use color_eyre::eyre::{Result, WrapErr};
use color_eyre::section::PanicMessage;
use owo_colors::OwoColorize;
use std::fs;
use std::process::Command;
use std::{fmt, panic::Location};
use structopt::StructOpt;
use tracing::instrument;

mod cli_args;
mod incremental;
mod toast;

use cli_args::Toast;
use incremental::{incremental_compile, IncrementalOpts};
use toast::breadbox::parse_import_map;

struct MyPanicMessage;

impl PanicMessage for MyPanicMessage {
    fn display(&self, pi: &std::panic::PanicInfo<'_>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", "The application panicked (crashed).".red())?;

        // Print panic message.
        let payload = pi
            .payload()
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| pi.payload().downcast_ref::<&str>().cloned())
            .unwrap_or("<non string panic payload>");

        write!(f, "Message:  ")?;
        writeln!(f, "{}", payload.cyan())?;

        // If known, print panic location.
        write!(f, "Location: ")?;
        if let Some(loc) = pi.location() {
            write!(f, "{}", loc.file().purple())?;
            write!(f, ":")?;
            write!(f, "{}", loc.line().purple())?;

            write!(
                f,
                "\n\nConsider reporting the bug at {}",
                custom_url(loc, payload)
            )?;
        } else {
            write!(f, "<unknown>")?;
        }

        Ok(())
    }
}

fn custom_url(location: &Location<'_>, message: &str) -> impl fmt::Display {
    "todo"
}

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

#[instrument]
fn main() -> Result<()> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();

    color_eyre::config::HookBuilder::default()
    .panic_message(MyPanicMessage)
    .panic_section("Please report the bug on github at https://github.com/christopherBiscardi/toast/issues/new with any context you have :)")
    .install()?;

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
    // println!("{:?}", opt);
    match opt {
        Toast::Incremental {
            debug,
            input_dir,
            output_dir,
        } => {
            let import_map = {
                let import_map_filepath = input_dir.join("public/web_modules/import-map.json");
                let contents = fs::read_to_string(&import_map_filepath).wrap_err_with(|| {
                    format!(
                        "Failed to read `import-map.json` from `{}`",
                        &import_map_filepath.display()
                    )
                })?;
                parse_import_map(&contents).wrap_err_with(|| {
                    format!(
                        "Failed to parse import map from content `{}` at `{}`",
                        contents,
                        &import_map_filepath.display()
                    )
                })?
            };

            incremental_compile(IncrementalOpts {
                debug,
                project_root_dir: input_dir.clone(),
                output_dir: match output_dir {
                    Some(v) => v,
                    None => {
                        let full_output_dir = input_dir.join("public");
                        std::fs::create_dir_all(&full_output_dir).wrap_err_with(|| {
                            format!(
                                "Failed create directories for path `{}`",
                                &full_output_dir.display()
                            )
                        })?;
                        full_output_dir
                            .canonicalize()
                            .wrap_err_with(|| {
                                format!("Failed canonicalize the output directory path")
                            })?
                            .to_path_buf()
                    }
                },
                npm_bin_dir,
                import_map,
            })
        }
    }
    // println!("{}", result)
    // .expect("failed to process file");
    // event.send(&mut client)
    // client.close();
}

#[cfg(feature = "capture-spantrace")]
fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
