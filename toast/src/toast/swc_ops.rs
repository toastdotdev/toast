use std::path::PathBuf;
use std::sync::Arc;
use tracing::instrument;

use swc::{
    self,
    config::{Config, JscConfig, JscTarget, Options, SourceMapsConfig, TransformConfig},
};
/*FoldWith,  VisitMut */
use swc_common::{
    chain,
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms::react;
use swc_ecma_visit::{Fold, FoldWith};

use crate::toast::{
    breadbox::ImportMap, cache::init, svg::SVGImportToComponent,
    swc_import_map_rewrite::SWCImportMapRewrite,
};

#[instrument]
pub fn compile_js_for_browser(
    source: String,
    filename: String,
    npm_bin_dir: PathBuf,
    import_map: ImportMap,
) -> String {
    let opts = &get_opts();
    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));

    let compiler = swc::Compiler::new(cm.clone(), handler.clone());

    let fm = cm.new_source_file(FileName::Custom(filename.clone()), source);

    let parsed_program = compiler.parse_js(fm.clone(), JscTarget::Es2020, get_syntax(), true, true);
    let built_config = compiler.config_for_file(opts, &FileName::Custom(filename.clone()));
    let post_transform_program = parsed_program.and_then(|program| {
        Ok(program.fold_with(&mut SWCImportMapRewrite {
            import_map: &import_map,
        }))
    });
    let result = compiler.transform(
        post_transform_program.unwrap().clone(),
        false,
        built_config.unwrap().pass,
    );
    // .and_then(|program| {
    //     if let Program::Module(mut module) = program {
    //         // println!("Matched {:?}!", i);
    //         module.visit_mut_with(&mut SVGImportToComponent {
    //             filepath: Path::new(&filename),
    //             npm_bin_dir: npm_bin_dir,
    //         });
    //         // program.print();
    //         return Ok(Program::Module(module));
    //     } else {
    //         // return error
    //         return Err(anyhow!("it's a script, dang"));
    //     }
    // });

    let output = compiler.print(&result, SourceMapsConfig::default(), None, false);
    return output.unwrap().code;
}

#[instrument]
pub fn compile_js_for_server(source: String, filename: String, npm_bin_dir: PathBuf) -> String {
    let opts = &get_opts();

    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));

    let compiler = swc::Compiler::new(cm.clone(), handler.clone());

    let fm = cm.new_source_file(FileName::Custom(filename.clone()), source);

    let parsed_program = compiler.parse_js(fm.clone(), JscTarget::Es2020, get_syntax(), true, true);
    let built_config = compiler.config_for_file(opts, &FileName::Custom(filename.clone()));

    let result = compiler.transform(
        parsed_program.unwrap().clone(),
        false,
        built_config.unwrap().pass,
    );
    // .and_then(|program| {
    //     if let Program::Module(mut module) = program {
    //         // println!("Matched {:?}!", i);
    //         module.visit_mut_with(&mut SVGImportToComponent {
    //             filepath: Path::new(&filename),
    //             npm_bin_dir: npm_bin_dir,
    //         });
    //         // program.print();
    //         return Ok(Program::Module(module));
    //     } else {
    //         // return error
    //         return Err(anyhow!("it's a script, dang"));
    //     }
    // });

    let output = compiler.print(&result, SourceMapsConfig::default(), None, false);
    return output.unwrap().code;
}

#[instrument]
fn get_opts() -> Options {
    return Options {
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
                        pragma: "Preact.t".to_string(),
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
    };
}

#[instrument]
fn get_syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        nullish_coalescing: true,
        optional_chaining: true,
        dynamic_import: true,
        ..Default::default()
    })
}
