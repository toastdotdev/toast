use color_eyre::Result;
use std::path::PathBuf;

use structopt::StructOpt;

fn abspath(input_dir: &std::ffi::OsStr) -> Result<PathBuf, std::ffi::OsString> {
    match PathBuf::from(input_dir).canonicalize() {
        Ok(dir) => Ok(dir),
        Err(_err) => Err(input_dir.into()),
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
        #[structopt(parse(try_from_os_str = abspath))]
        input_dir: PathBuf,

        /// Output file, stdout if not present
        #[structopt(parse(from_os_str))]
        output_dir: Option<PathBuf>,
    },
}
