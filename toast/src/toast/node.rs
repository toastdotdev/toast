use color_eyre::eyre::{eyre, Result};
use duct::cmd;
use indicatif::ProgressBar;
use std::{
    io::{prelude::*, BufReader},
    path::PathBuf,
    sync::Arc,
};
use tracing::instrument;

#[instrument]
pub fn render_to_html(
    dir_of_input_files: String,
    output_dir: String,
    filepaths: Vec<String>,
    npm_bin_dir: PathBuf,
    active_pb: Arc<ProgressBar>,
) -> Result<()> {
    let bin = npm_bin_dir.join("toast-render");
    let bin_str = bin
        .to_str()
        .ok_or_else(|| eyre!("failed to make npm bin into str"))?;
    let mut args: Vec<String> = vec![
        "--loader".to_owned(),
        "toast/src/loader.mjs".to_owned(),
        bin_str.to_owned(),
        dir_of_input_files,
        output_dir,
    ];
    args.extend(filepaths.iter().cloned());
    let output = cmd("node", args).stderr_to_stdout();
    if let Ok(reader) = output.reader() {
        let lines = BufReader::new(&reader).lines();
        for (i, line) in lines.filter_map(|m| m.ok()).enumerate() {
            // this magic number pulls off the warning
            if i > 1 {
                // if the progress bars are hidden, so is the
                // output from the pb.println function
                // so we use the println macro instead
                if active_pb.is_hidden() {
                    println!("{}", line)
                } else {
                    active_pb.println(line);
                }
            }
        }
        if let Ok(Some(output_status)) = &reader.try_wait() {
            if output_status.status.success() {
                return Ok(());
            } else if let Some(code) = output_status.status.code() {
                return Err(eyre!("renderToHtml node process exited with code {}", code));
            }
        }
    } else {
        return Err(eyre!("renderToHtml node process didn't start"));
    }
    Ok(())
}

#[instrument]
pub async fn source_data(
    toast_js_file: &PathBuf,
    npm_bin_dir: PathBuf,
    active_pb: Arc<ProgressBar>,
) -> Result<()> {
    // not a guarantee that toast.js will exist when node
    // goes to look for it: just a sanity check to not
    // execute Command if we don't need to
    if toast_js_file.exists() {
        let bin = npm_bin_dir.join("toast-source-data");
        let bin_str = bin
            .to_str()
            .ok_or_else(|| eyre!("failed to make npm bin into str"))?;
        let output = cmd!(
            "node",
            "--loader",
            "toast/src/loader.mjs",
            bin_str,
            "/var/tmp/toaster.sock",
            &toast_js_file
                .to_str()
                .ok_or_else(|| eyre!("failed to make toast_js_file into str"))?
        )
        .stderr_to_stdout();

        if let Ok(reader) = output.reader() {
            let lines = BufReader::new(&reader).lines();
            for (i, line) in lines.filter_map(|m| m.ok()).enumerate() {
                // this magic number pulls off the warning
                if i > 1 {
                    // if the progress bars are hidden, so is the
                    // output from the pb.println function
                    // so we use the println macro instead
                    if active_pb.is_hidden() {
                        println!("{}", line)
                    } else {
                        active_pb.println(line);
                    }
                }
            }
            if let Ok(Some(output_status)) = &reader.try_wait() {
                if output_status.status.success() {
                    return Ok(());
                } else if let Some(code) = output_status.status.code() {
                    return Err(eyre!("sourceData node process exited with code {}", code));
                }
            }
        } else {
            return Err(eyre!("sourceData node process didn't start"));
        }
        Ok(())
    } else {
        // toast file doesn't exist
        // skip running sourceData
        Ok(())
    }
}
