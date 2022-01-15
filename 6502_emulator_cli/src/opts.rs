use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opts {
    #[structopt(parse(from_os_str), short, long)]
    pub input: PathBuf
}

impl Opts {
    pub fn new() -> Self {
        Opts::from_args()
    }
}