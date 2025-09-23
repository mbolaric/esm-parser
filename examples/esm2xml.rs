mod helpers;

use crate::helpers::{ExportType, export, init_logging};
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(version, author = "Milan Bolaric", about = "Export ESM files (*.DDD) into XML", name = "esm2xml")]
pub struct Args {
    #[clap(short, long)]
    pub ddd_file: String,

    #[clap(global = true, short, long, default_value = "")]
    pub xml_file: String,
}

fn main() {
    init_logging();
    let args = Args::parse();
    export(&ExportType::Xml, &args.ddd_file, &args.xml_file);
}
