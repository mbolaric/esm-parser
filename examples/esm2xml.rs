mod helpers;

use clap::Parser;

use crate::helpers::{ExportType, export, init_logging};

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

fn output_format() -> ExportType {
    ExportType::Xml
}

fn main() {
    init_logging();
    let args = Args::parse();
    export(&output_format(), &args.ddd_file, &args.xml_file, &args.erca_gen1_file, &args.erca_gen2_file, args.pretty);
}

#[cfg(test)]
mod tests {
    use super::output_format;
    use crate::helpers::ExportType;

    #[test]
    fn selects_xml_output() {
        assert!(matches!(output_format(), ExportType::Xml));
    }
}
