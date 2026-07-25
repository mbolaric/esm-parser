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

    #[clap(global = true, short, long)]
    pub pretty: bool,

    #[clap(
        global = true,
        short = 'e',
        long,
        default_value = "",
        help = "Path to the Gen1 ERCA certificate (144 bytes). Used for Gen1 cards and the Gen1 application of combined cards."
    )]
    pub erca_gen1_file: String,

    #[clap(
        global = true,
        short = 'E',
        long,
        default_value = "",
        help = "Path to the Gen2 ERCA certificate (205 bytes). Used for Gen2 Driver/Workshop cards and their combined-card Gen2 application."
    )]
    pub erca_gen2_file: String,
}

fn main() {
    init_logging();
    let args = Args::parse();
    export(&ExportType::Json, &args.ddd_file, &args.json_file, &args.erca_gen1_file, &args.erca_gen2_file, args.pretty);
}
