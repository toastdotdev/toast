use anyhow::anyhow;
use walkdir::{DirEntry, WalkDir};

use std::{
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
use crate::toast::svg::SVGImportToComponent;

pub struct IncrementalOpts {
    pub debug: bool,
    pub input_dir: PathBuf,
    pub output_dir: Option<PathBuf>,
    pub npm_bin_dir: String,
}

pub fn incremental_compile(
    IncrementalOpts {
        debug,
        input_dir,
        output_dir,
        npm_bin_dir,
    }: IncrementalOpts,
) {
    let mut cache = init(npm_bin_dir.clone());
    let files: Vec<DirEntry> = WalkDir::new(&input_dir)
        .into_iter()
        .filter(|e| {
            return e.as_ref().map_or(false, |f| {
                f.file_name()
                    .to_str()
                    .map(|s| s.ends_with(".js"))
                    .unwrap_or(false)
            });
        })
        .map(|entry| {
            let e = entry.unwrap();
            let file_stuff = std::fs::read(e.path()).unwrap();
            // println!("{}", e.path().strip_prefix(&input_dir).unwrap().display());
            let source_id = e.path().strip_prefix(&input_dir).unwrap().to_str().unwrap();
            cache.set_source(source_id, file_stuff);
            cache.get_js_for_browser(source_id);
            cache.get_js_for_server(source_id);
            // println!("path: {:?}", file_stuff);
            e
        })
        .collect();

    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));

    let compiler = swc::Compiler::new(cm.clone(), handler.clone());

    let fm = cm
        .load_file(&input_dir.join("src/pages/index.js"))
        .expect("failed to load file");

    let result = compiler.process_js_file(
        fm,
        &Options {
            is_module: true,
            config: Some(Config {
                jsc: JscConfig {
                    syntax: Some(Syntax::Es(EsConfig {
                        jsx: true,
                        nullish_coalescing: true,
                        optional_chaining: true,
                        dynamic_import: true,
                        ..Default::default()
                    })),
                    transform: Some(TransformConfig {
                        react: react::Options {
                            pragma: "Preact.h".to_string(),
                            pragma_frag: "Preact.Fragment".to_string(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    match result {
        Ok(v) => println!("parsed file: {:?}", v),
        Err(e) => println!("error parsing file: {:?}", e),
    }
}
