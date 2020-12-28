use async_std::task;
use color_eyre::eyre::{eyre, Result, WrapErr};
use semver::Version;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, time::Instant};
use structopt::StructOpt;
use sys_info::{os_release, os_type};
use tracing::instrument;

use toast::{
    cli_args::Toast,
    esinstall::parse_import_map,
    incremental::{incremental_compile, IncrementalOpts},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[instrument]
fn get_npm_bin_dir() -> Result<PathBuf> {
    let npm_path = which::which("npm").expect("failed to get npm path");
    let output = Command::new(npm_path)
        .arg("bin")
        .output()
        .expect("failed to get npm bin dir");
    let possible_path = std::str::from_utf8(&output.stdout)?;
    Ok(PathBuf::from(possible_path.trim()))
}

#[instrument]
fn check_node_version() -> Result<()> {
    let minimum_required_node_major_version = Version {
        major: 14,
        minor: 0,
        patch: 0,
        pre: vec![],
        build: vec![],
    };

    let mut cmd = Command::new("node");
    cmd.arg("-v");
    let output = cmd
        .output()
        .wrap_err_with(|| "Failed to execute `node -v` Command and collect output")?;
    let version_string = std::str::from_utf8(&output.stdout)
        .wrap_err_with(|| "Failed to create utf8 string from node -v Command output")?;
    let version_string_trimmed = version_string.trim_start_matches('v');
    let current_node_version_result = Version::parse(version_string_trimmed);
    match current_node_version_result {
        Ok(current_node_version) => {
            if current_node_version < minimum_required_node_major_version {
                Err(eyre!(format!(
                    "node version {} doesn't meet the minimum required version {}",
                    current_node_version, minimum_required_node_major_version
                )))
            } else {
                Ok(())
            }
        }
        Err(_e) => Err(eyre!(format!(
            "Couldn't parse node version from trimmed version `{}`, original string is `{}`",
            version_string_trimmed, version_string
        ))),
    }
}

#[instrument]
fn main() -> Result<()> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();
    let start = Instant::now();

    color_eyre::config::HookBuilder::default()
        // .panic_message(MyPanicMessage)
        .issue_url("https://github.com/toastdotdev/toast/issues/new")
        .add_issue_metadata("version", VERSION)
        .add_issue_metadata(
            "os_type",
            os_type().unwrap_or_else(|_| "unavailable".to_string()),
        )
        .add_issue_metadata(
            "os_release",
            os_release().unwrap_or_else(|_| "unavailable".to_string()),
        )
        .install()?;

    check_node_version()?;
    let _ = fs::remove_file("/var/tmp/toaster.sock");
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
    let npm_bin_dir = get_npm_bin_dir()?;
    let opt = Toast::from_args();

    let result = match opt {
        Toast::Incremental {
            debug,
            input_dir,
            output_dir,
        } => {
            let import_map = {
                let import_map_filepath = output_dir
                    .clone()
                    .unwrap_or(input_dir.join("public"))
                    .join("web_modules")
                    .join("import-map.json");
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

            task::block_on(incremental_compile(IncrementalOpts {
                debug,
                project_root_dir: &input_dir,
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
                            .wrap_err_with(|| "Failed canonicalize the output directory path")?
                    }
                },
                npm_bin_dir,
                import_map,
            }))
        }
    };
    eprintln!("Toast executed in {:?}", start.elapsed());
    result
    // .expect("failed to process file");
    // event.send(&mut client)
    // client.close();
}

#[cfg(feature = "capture-spantrace")]
fn install_tracing() {
    println!("capturing spantraces");
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
