use crate::toast::{
    breadbox::ImportMap,
    cache::init,
    node::{render_to_html, source_data},
    sources::{Source, SourceKind},
};
use async_std::task;
use color_eyre::eyre::{eyre, Result, WrapErr};
use crossbeam::{unbounded, Sender};
use fs_extra::dir::{copy, CopyOptions};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::value::Value;
use std::sync::Arc;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process,
};
use tracing::instrument;
use walkdir::WalkDir;

use crate::toast::{
    cache::Cache,
    internal_api::{ModuleSpec, SetDataForSlug},
};

#[derive(Debug)]
pub struct IncrementalOpts<'a> {
    pub debug: bool,
    pub project_root_dir: &'a PathBuf,
    pub output_dir: PathBuf,
    pub npm_bin_dir: PathBuf,
    pub toast_module_path: Option<PathBuf>,
    pub import_map: ImportMap,
}

#[derive(Debug)]
struct OutputFile {
    dest: String,
}

#[derive(Clone)]
struct TideSharedState {
    tx: Sender<Event>,
    output_dir: PathBuf,
    create_page_progress_bar: Arc<ProgressBar>,
}
#[derive(Debug, Clone)]
enum Event {
    Set(SetDataForSlug),
}

#[instrument]
pub async fn incremental_compile(opts: IncrementalOpts<'_>) -> Result<()> {
    let IncrementalOpts {
        debug,
        project_root_dir,
        output_dir,
        npm_bin_dir,
        toast_module_path,
        import_map,
    } = opts;
    let tmp_dir = {
        let mut dir = project_root_dir.clone();
        dir.push(".tmp");
        dir
    };
    std::fs::create_dir_all(&tmp_dir).wrap_err_with(|| {
        format!(
            "Failed to create directories for tmp_dir `{}`. Can not compile files into directory that doesn't exist, exiting.",
            &tmp_dir.display()
        )
    })?;

    let create_pages_pb = Arc::new(ProgressBar::new_spinner());
    create_pages_pb.enable_steady_tick(120);
    create_pages_pb.set_style(
        ProgressStyle::default_spinner()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&["▹▹▹", "▸▹▹", "▹▸▹", "▹▹▸", "▪▪▪"])
            .template("{spinner:.blue} [{elapsed}] {pos} {wide_msg}"),
    );
    create_pages_pb.set_message("fetching data...");
    create_pages_pb.tick();
    // channel to listen for createPage events
    let (tx, rx) = unbounded();
    // create incremental cache db
    let mut cache = init(npm_bin_dir.clone());

    // boot server
    let mut app = tide::with_state(TideSharedState {
        tx,
        output_dir: output_dir.clone(),
        create_page_progress_bar: create_pages_pb.clone(),
    });
    let sock = format!("http+unix://{}", "/var/tmp/toaster.sock");
    app.at("/").get(|_| async { Ok("ready") });
    app.at("/set-data-for-slug")
        .post(|mut req: tide::Request<TideSharedState>| async move {
            let mut data: SetDataForSlug = req.body_json().await?;
            req.state()
                .create_page_progress_bar
                .set_message(data.slug.as_str());
            req.state().create_page_progress_bar.inc(1);

            data.normalize();

            req.state().tx.send(Event::Set(data))?;
            Ok("ok")
        });
    app.at("/terminate").get(|_| async {
        let _result = fs::remove_file("/var/tmp/toaster.sock");
        process::exit(0);
        #[allow(unreachable_code)]
        Ok("done")
    });
    let server = task::spawn(app.listen(sock));

    let files_by_source_id = compile_src_files(
        IncrementalOpts {
            debug,
            project_root_dir: &project_root_dir,
            output_dir: output_dir.clone(),
            npm_bin_dir: npm_bin_dir.clone(),
            toast_module_path: toast_module_path.clone(),
            import_map: import_map.clone(),
        },
        &mut cache,
        &tmp_dir,
    )?;
    // render_src_pages()?;
    let file_list = files_by_source_id
        .iter()
        .map(|(_, output_file)| output_file.dest.clone())
        .collect::<Vec<String>>();
    let _data_from_user = source_data(
        &project_root_dir.join("toast.js"),
        npm_bin_dir.clone(),
        toast_module_path.clone(),
        create_pages_pb.clone(),
    )
    .await?;

    let _maybe_gone = server.cancel();
    let _result = fs::remove_file("/var/tmp/toaster.sock");
    create_pages_pb.abandon_with_message("pages created");

    let set_data_events: Vec<Event> = rx.try_iter().collect();
    let event_len: u64 = set_data_events.len() as u64;
    let compile_pb = Arc::new(ProgressBar::new_spinner());
    compile_pb.enable_steady_tick(120);
    compile_pb.set_length(event_len);
    compile_pb.set_style(
        ProgressStyle::default_spinner()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&["▹▹▹", "▸▹▹", "▹▸▹", "▹▹▸", "▪▪▪"])
            .template("{spinner:.blue} [{elapsed}] {pos}/{len} {wide_msg}"),
    );
    compile_pb.set_message("compiling...");
    compile_pb.tick();

    for x in set_data_events.clone() {
        match x {
            Event::Set(set) => {
                compile_pb.inc(1);
                compile_pb.set_message(set.slug.as_str());
                let slug_filepath = set.slug_as_relative_filepath();
                let mut output_path_js = set.slug_as_relative_filepath();
                output_path_js.set_extension("js");

                // if there is a component, set the source in the incremental cache
                // if it's a filepath, we need to figure out how to handle it. I think
                // that we just need to make sure the filepaths are relative to the project
                // root so that the incremental cache ids for the sources line up when
                // we go to render it out or whatnot
                match &set.component {
                    None => {}
                    Some(ModuleSpec::NoModule) => {
                        panic!("no-module is not implemented for components yet")
                    }
                    Some(ModuleSpec::File { path: _ }) => {
                        panic!("Filepaths are not implemented yet")
                    }
                    Some(ModuleSpec::Source { code }) => {
                        cache.set_source(
                            &set.slug,
                            Source {
                                source: code.to_string(),
                                kind: SourceKind::Raw,
                            },
                        );
                        compile_js(
                            &set.slug,
                            &OutputFile {
                                dest: output_path_js.display().to_string(),
                            },
                            IncrementalOpts {
                                debug,
                                project_root_dir: &project_root_dir,
                                output_dir: output_dir.clone(),
                                npm_bin_dir: npm_bin_dir.clone(),
                                toast_module_path: toast_module_path.clone(),
                                import_map: import_map.clone(),
                            },
                            &mut cache,
                            &tmp_dir,
                        )?;
                    }
                }
                match &set.data {
                    Some(Value::Null) => {
                        // if null, do nothing for now. In the future null
                        // will cause us to overlay a tombstone on this layer
                        // similar to an overlay filesystem, resulting in no data
                        // for the page.
                    }
                    Some(v) => {
                        // we write the files out to disk here today,
                        // we should probably put them in the incremental cache first
                        // so that files can depend on them via derived queries
                        let mut json_path = output_dir.join(slug_filepath);
                        json_path.set_extension("json");
                        std::fs::create_dir_all(&json_path.parent().unwrap())?;
                        fs::write(json_path, v.to_string())?
                    }
                    None => {}
                }
                match &set.wrapper {
                    Some(_) => {
                        panic!("set.wrapper is not implemented yet");
                    }
                    None => {}
                }
            }
        };
    }
    compile_pb.abandon_with_message("remote sources compiled");

    let remote_file_list: Vec<String> = set_data_events
        .iter()
        .filter_map(|Event::Set(set)| match set.component {
            None => None,
            Some(_) => {
                let mut js_filepath = set.slug_as_relative_filepath();
                js_filepath.set_extension("js");
                Some(js_filepath.display().to_string())
            }
        })
        .collect();
    let mut list: Vec<String> = file_list
        .clone()
        .iter()
        .filter(|f| f.starts_with("src/pages") || f.starts_with(r"src\pages"))
        .cloned()
        .collect();
    list.extend(remote_file_list);

    let render_pb = Arc::new(ProgressBar::new_spinner());
    render_pb.enable_steady_tick(120);
    render_pb.set_style(
        ProgressStyle::default_spinner()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&["▹▹▹", "▸▹▹", "▹▸▹", "▹▹▸", "▪▪▪"])
            .template("{spinner:.blue} [{elapsed}] {wide_msg}"),
    );
    render_pb.set_message("rendering html...");
    render_pb.tick();
    render_to_html(
        tmp_dir.into_os_string().into_string().unwrap(),
        output_dir.into_os_string().into_string().unwrap(),
        list,
        npm_bin_dir,
        toast_module_path,
        render_pb.clone(),
    )?;
    render_pb.abandon_with_message("html rendered");

    // # copy static dir to public dir
    //
    // * copy_inside seems to be for copying the whole `static` folder to
    //   `public/static`.
    // * `content_only` seems to be for copying `static/*` into `public/`
    let options = CopyOptions {
        copy_inside: false,
        overwrite: true,
        content_only: true,
        ..CopyOptions::new()
    };
    let static_dir = project_root_dir.join("static");
    let public_dir = project_root_dir.join("public");
    if static_dir.exists() && public_dir.exists() {
        copy(static_dir, public_dir, &options)?;
    }

    Ok(())
}

#[instrument(skip(cache))]
fn compile_src_files(
    opts: IncrementalOpts,
    cache: &mut Cache,
    tmp_dir: &PathBuf,
) -> Result<HashMap<String, OutputFile>> {
    let IncrementalOpts {
        debug,
        project_root_dir,
        output_dir,
        npm_bin_dir,
        toast_module_path,
        import_map,
    } = opts;
    let files_by_source_id: HashMap<String, OutputFile> =
        WalkDir::new(&project_root_dir.join("src"))
            .into_iter()
            // only scan .js files
            .filter(|result| {
                result.as_ref().map_or(false, |dir_entry| {
                    dir_entry
                        .file_name()
                        .to_str()
                        .map(|filename| filename.ends_with(".js"))
                        .unwrap_or(false)
                })
            })
            // insert source files into cache and return a
            // HashMap so we can access the entries and such later
            // by source_id
            .fold(HashMap::new(), |mut map, entry| {
                let e = entry.unwrap();
                let path_buf = e.path().to_path_buf();
                let file_stuff = cache.read(path_buf.clone());
                let source_id = e
                    .path()
                    .strip_prefix(&project_root_dir)
                    .unwrap()
                    .to_str()
                    .unwrap();
                cache.set_source(
                    source_id,
                    Source {
                        source: file_stuff,
                        kind: SourceKind::File {
                            relative_path: path_buf,
                        },
                    },
                );

                map.entry(String::from(source_id)).or_insert(OutputFile {
                    dest: source_id.to_string(),
                });
                map
            });
    for (source_id, output_file) in files_by_source_id.iter() {
        compile_js(
            source_id,
            output_file,
            IncrementalOpts {
                debug,
                project_root_dir: &project_root_dir,
                output_dir: output_dir.clone(),
                npm_bin_dir: npm_bin_dir.clone(),
                toast_module_path: toast_module_path.clone(),
                import_map: import_map.clone(),
            },
            cache,
            &tmp_dir,
        )?;
    }
    Ok(files_by_source_id)
}

#[instrument(skip(cache))]
fn compile_js(
    source_id: &str,
    output_file: &OutputFile,
    opts: IncrementalOpts,
    cache: &mut Cache,
    tmp_dir: &PathBuf,
) -> Result<()> {
    let IncrementalOpts {
        debug: _,
        project_root_dir: _,
        output_dir,
        npm_bin_dir: _,
        toast_module_path: _,
        import_map,
    } = opts;
    let browser_output_file = output_dir.join(Path::new(&output_file.dest));
    let js_browser = cache.get_js_for_browser(source_id, import_map);
    let file_dir = browser_output_file.parent().ok_or(eyre!(format!(
        "could not get .parent() directory for `{}`",
        &browser_output_file.display()
    )))?;
    std::fs::create_dir_all(&file_dir).wrap_err_with(|| {
        format!(
            "Failed to create parent directories for `{}`. ",
            &browser_output_file.display()
        )
    })?;
    let _res = std::fs::write(&browser_output_file, js_browser).wrap_err_with(|| {
        format!(
            "Failed to write browser JS file for `{}`. ",
            &browser_output_file.display()
        )
    })?;

    let js_node = cache.get_js_for_server(source_id);
    let mut node_output_file = tmp_dir.clone();
    node_output_file.push(&output_file.dest);
    // node_output_file.set_extension("mjs");
    let file_dir = node_output_file.parent().ok_or(eyre!(format!(
        "could not get .parent() directory for `{}`",
        &node_output_file.display()
    )))?;
    std::fs::create_dir_all(&file_dir).wrap_err_with(|| {
        format!(
            "Failed to create parent directories for `{}`. ",
            &browser_output_file.display()
        )
    })?;
    let _node_res = std::fs::write(&node_output_file, js_node).wrap_err_with(|| {
        format!(
            "Failed to write node JS file for `{}`. ",
            &node_output_file.display()
        )
    })?;
    Ok(())
}
