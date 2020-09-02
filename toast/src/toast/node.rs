use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn render_to_html(
    src_dir: String,
    output_dir: String,
    filepaths: Vec<String>,
    npm_bin_dir: String,
) {
    println!("{:?}", env::current_dir().unwrap().join("toast-render.js"));
    let mut cmd = Command::new(env::current_dir().unwrap().join("toast-render.js"));
    cmd.arg(src_dir).arg(output_dir);
    for arg in filepaths {
        cmd.arg(arg);
    }
    /*
    .arg("run")
     */
    let output = cmd.output();
    println!("{:?}", output);
}
