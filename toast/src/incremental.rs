use crate::toast::{
    breadbox::ImportMap,
    cache::init,
    node::{render_to_html, source_data},
    sources::{Source, SourceKind},
};
use async_std::task;
use color_eyre::{
    eyre::{eyre, Report, Result, WrapErr},
    Section,
};
use crossbeam::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process,
    sync::{Arc, Mutex},
};
use tracing::instrument;
use walkdir::WalkDir;

use crate::toast::cache::Cache;

#[derive(Debug)]
pub struct IncrementalOpts<'a> {
    pub debug: bool,
    pub project_root_dir: &'a PathBuf,
    pub output_dir: PathBuf,
    pub npm_bin_dir: String,
    pub import_map: ImportMap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CreatePage {
    module: String,
    slug: String,
    data: serde_json::Value,
}
#[derive(Debug)]
struct OutputFile {
    dest: String,
}

#[derive(Clone)]
struct TideSharedState {
    tx: Sender<Event>,
}
#[derive(Debug, Clone)]
enum Event {
    CreatePage(CreatePage),
}

// #[instrument]
pub async fn incremental_compile(
    IncrementalOpts {
        debug,
        project_root_dir,
        output_dir,
        npm_bin_dir,
        import_map,
    }: IncrementalOpts<'_>,
) -> Result<()> {
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

    // channel to listen for createPage events
    let (tx, rx) = unbounded();
    // create incremental cache db
    let mut cache = init(npm_bin_dir.clone());

    // boot server
    let mut app = tide::with_state(TideSharedState { tx });
    let sock = format!("http+unix://{}", "/var/tmp/toaster.sock");
    app.at("/").get(|_| async { Ok("ready") });
    app.at("/create-page")
        .post(|mut req: tide::Request<TideSharedState>| async move {
            let create_page: CreatePage = req.body_json().await?;
            &req.state()
                .tx
                .send(Event::CreatePage(create_page.clone()))?;
            // println!("{:?}", create_page);
            Ok("thanks")
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

    let data_from_user = source_data(&project_root_dir.join("toast.mjs")).await;

    let maybe_gone = server.cancel();
    let _result = fs::remove_file("/var/tmp/toaster.sock");

    let v: Vec<Event> = rx.try_iter().collect();
    for x in v.clone() {
        match x {
            Event::CreatePage(CreatePage { module, data, slug }) => {
                &cache.set_source(
                    &slug,
                    Source {
                        source: module,
                        kind: SourceKind::Raw,
                    },
                );
            }
        };
    }
    for x in v.clone() {
        match x {
            Event::CreatePage(CreatePage { module, data, slug }) => {
                compile_js(
                    &slug,
                    &OutputFile {
                        dest: format!("{}.js", slug.trim_matches('/')),
                    },
                    IncrementalOpts {
                        debug,
                        project_root_dir: &project_root_dir,
                        output_dir: output_dir.clone(),
                        npm_bin_dir: npm_bin_dir.clone(),
                        import_map: import_map.clone(),
                    },
                    &mut cache,
                    &tmp_dir,
                )?;
            }
        }
    }
    let remote_file_list: Vec<String> = v
        .iter()
        .map(|Event::CreatePage(CreatePage { slug, .. })| format!("{}.js", slug.trim_matches('/')))
        .collect();
    // vec![].file_list.append(remote_file_list)
    let mut list = file_list.clone();
    list.extend(remote_file_list);
    render_to_html(
        tmp_dir.into_os_string().into_string().unwrap(),
        output_dir.into_os_string().into_string().unwrap(),
        list,
        npm_bin_dir,
    )?;
    Ok(())
}

// #[instrument(skip(cache))]
fn compile_src_files(
    IncrementalOpts {
        debug,
        project_root_dir,
        output_dir,
        npm_bin_dir,
        import_map,
    }: IncrementalOpts,
    cache: &mut Cache,
    tmp_dir: &PathBuf,
) -> Result<HashMap<String, OutputFile>> {
    let files_by_source_id: HashMap<String, OutputFile> =
        WalkDir::new(&project_root_dir.join("src"))
            .into_iter()
            // only scan .js files
            .filter(|result| {
                return result.as_ref().map_or(false, |dir_entry| {
                    dir_entry
                        .file_name()
                        .to_str()
                        .map(|filename| filename.ends_with(".js"))
                        .unwrap_or(false)
                });
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
                            relative_path: path_buf.clone(),
                        },
                    },
                );
                panic!("at the disco");

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
                import_map: import_map.clone(),
            },
            cache,
            &tmp_dir,
        )?;
    }
    Ok(files_by_source_id)
}

fn compile_js(
    source_id: &str,
    output_file: &OutputFile,
    IncrementalOpts {
        debug,
        project_root_dir,
        output_dir,
        npm_bin_dir,
        import_map,
    }: IncrementalOpts,
    cache: &mut Cache,
    tmp_dir: &PathBuf,
) -> Result<()> {
    let browser_output_file = output_dir.join(Path::new(&output_file.dest));
    let js_browser = cache.get_js_for_browser(source_id, import_map.clone());
    std::fs::create_dir_all(&browser_output_file.parent().unwrap());
    let _res = std::fs::write(&browser_output_file, js_browser).wrap_err_with(|| {
        format!(
            "Failed to write browser JS file for `{}`. ",
            &browser_output_file.display()
        )
    })?;

    let js_node = cache.get_js_for_server(source_id);
    let mut node_output_file = tmp_dir.clone();
    node_output_file.push(&output_file.dest);
    node_output_file.set_extension("mjs");
    // TODO: handle directory creation errors gracefully
    std::fs::create_dir_all(&node_output_file.parent().unwrap());
    let _node_res = std::fs::write(&node_output_file, js_node).wrap_err_with(|| {
        format!(
            "Failed to write node JS file for `{}`. ",
            &node_output_file.display()
        )
    })?;
    Ok(())
}
