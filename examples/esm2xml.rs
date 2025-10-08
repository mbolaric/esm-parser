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

    #[clap(global = true, short, long)]
    pub pretty: bool,

    #[clap(
        global = true,
        short,
        long,
        default_value = "",
        help = "Path for ERCA PK *.bin file (144 bytes). When this file is provided we verify vertificates, Work only for Gen1 Version."
    )]
    pub erca_gen1_file: String,

    #[clap(
        global = true,
        short,
        long,
        default_value = "",
        help = "Path for ERCA PK *.bin file (205 bytes). When this file is provided we verify vertificates, Work for Gen2 Version."
    )]
    pub erca_gen2_file: String,
}

fn main() {
    init_logging();
    let args = Args::parse();
    export(&ExportType::Json, &args.ddd_file, &args.xml_file, &args.erca_gen1_file, &args.erca_gen2_file, args.pretty);
}
