use anyhow::anyhow;
use walkdir::{DirEntry, WalkDir};

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

// use swc;
use swc::{
    self,
    config::{Config, JscConfig, JscTarget, Options, SourceMapsConfig, TransformConfig},
};
/*FoldWith,  VisitMut */
use swc_ecma_visit::VisitMutWith;

use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
};
use swc_ecma_ast::Program;
// use swc_ecma_ast::Program;
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms::react;

use crate::toast::cache::init;
use crate::toast::svg::SVGImportToComponent;

pub fn compile_js_for_browser(source: String, filename: String, npm_bin_dir: String) -> String {
    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));

    let compiler = swc::Compiler::new(cm.clone(), handler.clone());

    let fm = cm.new_source_file(FileName::Custom(filename), source);
    // .load_file(&input_dir.join("src/pages/index.js"))
    // .expect("failed to load file");

    let _parsed_program = compiler
        .parse_js(
            fm.clone(),
            JscTarget::Es2020,
            Syntax::Es(EsConfig {
                jsx: true,
                nullish_coalescing: true,
                optional_chaining: true,
                dynamic_import: true,
                ..Default::default()
            }),
            true,
            true,
        )
        .and_then(|program| {
            if let Program::Module(mut module) = program {
                // println!("Matched {:?}!", i);
                module.visit_mut_with(&mut SVGImportToComponent {
                    filepath: Path::new("test-toast-site/src/pages/index.js"),
                    npm_bin_dir: npm_bin_dir,
                });
                // program.print();
                return Ok(Program::Module(module));
            } else {
                // return error
                return Err(anyhow!("it's a script, dang"));
            }
        });
    let output = compiler.print(
        &_parsed_program.unwrap(),
        SourceMapsConfig::default(),
        None,
        true,
    );
    return output.unwrap().code;
}
