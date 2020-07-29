use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use structopt::StructOpt;
// use swc;
use swc::{
    self,
    common::{
        errors::{ColorConfig, Handler},
        SourceMap,
    },
    config::{Config, JscConfig, Options},
    ecmascript::parser::{EsConfig, Syntax},
};

#[derive(Debug, StructOpt)]
#[structopt(name = "toast", about = "The best place to stack your JAM")]
enum Toast {
    Incremental {
        /// Activate debug mode
        // short and long flags (-d, --debug) will be deduced from the field's name
        #[structopt(short, long)]
        debug: bool,

        /// Input file
        #[structopt(parse(from_os_str))]
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

    let result = compiler.process_js_file(
        fm,
        &Options {
            is_module: true,
            config: Some(Config {
                jsc: JscConfig {
                    syntax: Some(Syntax::Es(EsConfig {
                        jsx: true,
                        ..Default::default()
                    })),
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
