use color_eyre::{eyre::eyre, Result};
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::instrument;

#[instrument]
fn abspath(input_dir: &str) -> Result<PathBuf> {
    match PathBuf::from(input_dir).canonicalize() {
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
    Incremental {
        /// Activate debug mode
        // short and long flags (-d, --debug) will be deduced from the field's name
        #[structopt(short, long)]
        debug: bool,

        /// Input file
        #[structopt(parse(try_from_str = abspath))]
        input_dir: PathBuf,

        /// Output file, stdout if not present
        #[structopt(parse(from_os_str))]
        output_dir: Option<PathBuf>,
    },
}
