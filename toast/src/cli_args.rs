use color_eyre::{eyre::eyre, Result};
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::instrument;

#[instrument]
fn abspath(input_dir: &str) -> Result<PathBuf> {
    match dunce::canonicalize(input_dir) {
        Ok(dir) => Ok(dir),
        Err(_err) => Err(eyre!(
            "Could not find directory `{}` for input_dir from {}",
            input_dir,
            env::current_dir().unwrap().display()
        )),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "toast", about = "The best place to stack your JAM")]
pub enum Toast {
    /// Incrementally build your input directory
    #[structopt(name = "incremental")]
    Incremental {
        /// Activate debug mode
        #[structopt(short, long)]
        debug: bool,

        /// The directory of your Toast site
        #[structopt(parse(try_from_str = abspath))]
        input_dir: PathBuf,

        /// Output directory, "./public" if not present
        #[structopt(parse(from_os_str))]
        output_dir: Option<PathBuf>,
    },
}
