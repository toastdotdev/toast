use anyhow::anyhow;
use walkdir::{DirEntry, WalkDir};

use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

// use swc;
use swc::{
    self,
    config::{Config, JscConfig, JscTarget, Options, TransformConfig},
};
/*FoldWith,  VisitMut */
use swc_ecma_visit::VisitMutWith;

use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_ast::Program;
// use swc_ecma_ast::Program;
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms::react;

use crate::toast::cache::init;
use crate::toast::node::render_to_html;
use crate::toast::svg::SVGImportToComponent;

pub struct IncrementalOpts {
    pub debug: bool,
    pub project_root_dir: PathBuf,
    pub output_dir: PathBuf,
    pub npm_bin_dir: String,
}

#[derive(Debug)]
struct OutputFile {
    dest: String,
}

pub fn incremental_compile(
    IncrementalOpts {
        debug,
        project_root_dir,
        output_dir,
        npm_bin_dir,
    }: IncrementalOpts,
) {
    let tmp_dir = {
        let mut dir = project_root_dir.clone();
        dir.push(".tmp");
        dir
    };
    std::fs::create_dir_all(&tmp_dir);

    let mut cache = init(npm_bin_dir.clone());
    let files_by_source_id: HashMap<String, OutputFile> =
        WalkDir::new(&project_root_dir.join("src"))
            .into_iter()
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
                let file_stuff = std::fs::read(e.path()).unwrap();
                let source_id = e
                    .path()
                    .strip_prefix(&project_root_dir)
                    .unwrap()
                    .to_str()
                    .unwrap();
                cache.set_source(source_id, file_stuff);
                map.entry(String::from(source_id)).or_insert(OutputFile {
                    dest: source_id.to_string(),
                });
                map
            });

    for (source_id, output_file) in files_by_source_id.iter() {
        let browser_output_file = output_dir.join(Path::new(&output_file.dest));
        let js_browser = cache.get_js_for_browser(source_id);
        std::fs::create_dir_all(browser_output_file.parent().unwrap());
        let res = std::fs::write(browser_output_file, js_browser);

        let js_node = cache.get_js_for_server(source_id);
        let mut node_output_file = tmp_dir.clone();
        node_output_file.push(&output_file.dest);
        // TODO
        node_output_file.set_extension("mjs");
        std::fs::create_dir_all(node_output_file.parent().unwrap());
        let node_res = std::fs::write(node_output_file, js_node);
    }

    let file_list = files_by_source_id
        .iter()
        .map(|(_, output_file)| output_file.dest.clone())
        .collect::<Vec<String>>();
    render_to_html(
        tmp_dir.into_os_string().into_string().unwrap(),
        output_dir.into_os_string().into_string().unwrap(),
        file_list,
        npm_bin_dir,
    );
}

// let cm = Arc::<SourceMap>::default();
// let handler = Arc::new(Handler::with_tty_emitter(
//     ColorConfig::Auto,
//     true,
//     false,
//     Some(cm.clone()),
// ));

// let compiler = swc::Compiler::new(cm.clone(), handler.clone());

// let fm = cm
//     .load_file(&project_root_dir.join("src/pages/index.js"))
//     .expect("failed to load file");

// let result = compiler.process_js_file(
//     fm,
//     &Options {
//         is_module: true,
//         config: Some(Config {
//             jsc: JscConfig {
//                 syntax: Some(Syntax::Es(EsConfig {
//                     jsx: true,
//                     nullish_coalescing: true,
//                     optional_chaining: true,
//                     dynamic_import: true,
//                     ..Default::default()
//                 })),
//                 transform: Some(TransformConfig {
//                     react: react::Options {
//                         pragma: "Preact.t".to_string(),
//                         pragma_frag: "Preact.Fragment".to_string(),
//                         ..Default::default()
//                     },
//                     ..Default::default()
//                 }),
//                 ..Default::default()
//             },
//             ..Default::default()
//         }),
//         ..Default::default()
//     },
// );

// match result {
//     Ok(v) => println!("parsed file: {:?}", v),
//     Err(e) => println!("error parsing file: {:?}", e),
// }
