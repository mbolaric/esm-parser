mod helpers;

use crate::helpers::{ExportType, export, init_logging};
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(version, author = "Milan Bolaric", about = "Export ESM files (*.DDD) into JSON", name = "esm2json")]
pub struct Args {
    #[clap(short, long)]
    pub ddd_file: String,

    #[clap(global = true, short, long, default_value = "")]
    pub json_file: String,
}

fn main() {
    init_logging();
    let args = Args::parse();
    export(&ExportType::Json, &args.ddd_file, &args.json_file);
}
