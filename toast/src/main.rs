use std::process::Command;

use structopt::StructOpt;

mod cli_args;
mod incremental;
mod toast;

use cli_args::Toast;
use incremental::{incremental_compile, IncrementalOpts};

fn get_npm_bin_dir() -> String {
    let output = Command::new("npm")
        .arg("bin")
        .output()
        .expect("failed to execute process");
    match String::from_utf8(output.stdout) {
        Ok(output_string) => output_string,
        Err(e) => {
            println!("utf8 conversion error {}", e);
            panic!("npm bin location could not be found, exiting")
        }
    }
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
    let npm_bin_dir = get_npm_bin_dir();
    let opt = Toast::from_args();
    println!("{:?}", opt);

    match opt {
        Toast::Incremental {
            debug,
            input_dir,
            output_dir,
        } => incremental_compile(IncrementalOpts {
            debug,
            input_dir,
            output_dir,
            npm_bin_dir,
        }),
    }
    // println!("{}", result)
    // .expect("failed to process file");
    // event.send(&mut client)
    // client.close();
}
