use anyhow::{anyhow, Result};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use structopt::StructOpt;
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

mod toast;

use toast::svg::SVGImportToComponent;

fn abspath(input_dir: &std::ffi::OsStr) -> Result<PathBuf, std::ffi::OsString> {
    match PathBuf::from(input_dir).canonicalize() {
        Ok(dir) => Ok(dir),
        Err(v) => Err(input_dir.into()),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "toast", about = "The best place to stack your JAM")]
enum Toast {
    Incremental {
        /// Activate debug mode
        // short and long flags (-d, --debug) will be deduced from the field's name
        #[structopt(short, long)]
        debug: bool,

        /// Input file
        #[structopt(parse(try_from_os_str = abspath))]
        input_dir: PathBuf,

        /// Output file, stdout if not present
        #[structopt(parse(from_os_str))]
        output_dir: Option<PathBuf>,
    },
}

fn main() {
    // let client = libhoney::init(libhoney::Config {
    //     options: libhoney::client::Options {
    //         api_key: "YOUR_API_KEY".to_string(),
    //         dataset: "honeycomb-rust-example".to_string(),
    //         ..libhoney::client::Options::default()
    //     },
    //     transmission_options: libhoney::transmission::Options::default(),
    // });
    // event := builder.new_event()
    // event.add_field("key", Value::String("val".to_string())), event.add(data)
    let opt = Toast::from_args();
    println!("{:?}", opt);

    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));

    let compiler = swc::Compiler::new(cm.clone(), handler.clone());

    let fm = cm
        .load_file(Path::new("test-toast-site/src/pages/index.js"))
        .expect("failed to load file");

    // Custom Transforms test
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
                });
                return Ok(Program::Module(module));
            } else {
                // return error
                return Err(anyhow!("it's a script, dang"));
            }
        });

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
    // println!("{}", result)
    // .expect("failed to process file");
    // event.send(&mut client)
    // client.close();
}
