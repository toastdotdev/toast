use color_eyre::eyre::{eyre, Result};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tracing::instrument;

#[instrument]
pub fn render_to_html(
    dir_of_input_files: String,
    output_dir: String,
    filepaths: Vec<String>,
    npm_bin_dir: PathBuf,
) -> Result<()> {
    let bin = npm_bin_dir.join("toast-render");
    let mut cmd = Command::new("node");
    let bin_str = bin
        .to_str()
        .ok_or_else(|| eyre!("failed to make npm bin into str"))?;
    cmd.args(&[
        "--no-warnings",
        "--loader",
        "toastrs/src/loader.mjs",
        bin_str,
        &dir_of_input_files,
        &output_dir,
    ]);
    for arg in filepaths {
        cmd.arg(arg);
    }
    let output = cmd.output()?;
    let _ = std::io::stdout().write_all(&output.stdout);
    let _ = std::io::stderr().write_all(&output.stderr);
    Ok(())
}

#[instrument]
pub async fn source_data(toast_js_file: &PathBuf, npm_bin_dir: PathBuf) -> Result<()> {
    // not a guarantee that toast.js will exist when node
    // goes to look for it: just a sanity check to not
    // execute Command if we don't need to
    if toast_js_file.exists() {
        let bin = npm_bin_dir.join("toast-source-data");
        let mut cmd = Command::new("node");
        let bin_str = bin
            .to_str()
            .ok_or_else(|| eyre!("failed to make npm bin into str"))?;
        cmd.args(&[
            "--no-warnings",
            "--loader",
            "toastrs/src/loader.mjs",
            bin_str,
            "/var/tmp/toaster.sock",
            &toast_js_file
                .to_str()
                .ok_or_else(|| eyre!("failed to make toast_js_file into str"))?,
        ]);
        let output = cmd.output()?;
        // TODO: move stdout/stderr around so it's not just dumping to console
        let _ = std::io::stdout().write_all(&output.stdout);
        let _ = std::io::stderr().write_all(&output.stderr);
        Ok(())
    } else {
        Ok(())
    }
}
