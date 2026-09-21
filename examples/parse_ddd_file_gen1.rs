mod helpers;

use esm_parser::{Export, TachographData, parse_from_file};
use helpers::init_logging;
use log::debug;

fn main() {
    init_logging();

    let args: Vec<String> = std::env::args().collect();
    let default_path = "./examples/data/Card0001.DDD";
    let file_path = if args.len() > 1 {
        &args[1]
    } else if std::path::Path::new(default_path).exists() {
        default_path
    } else {
        eprintln!("Usage: cargo run --example parse_ddd_file_gen1 -- <path/to/card.DDD>");
        eprintln!("Error: No DDD file path provided, and default test file does not exist.");
        std::process::exit(1);
    };

    match parse_from_file(file_path) {
        Ok(data) => match data {
            TachographData::CardGen1(inner) => {
                println!("CardGen1:");
                println!("{:#?}", inner);
                println!("{:?}", inner.to_json());
                println!("{:?}", inner.to_xml());
            }
            TachographData::CardGen2(inner) => {
                println!("CardGen2:");
                println!("{:#?}", inner);
                println!("{:?}", inner.to_json());
            }
            TachographData::VUGen1(inner) => {
                println!("VUGen1:");
                println!("{:#?}", inner);
                println!("{:?}", inner.to_json());
            }
            TachographData::VUGen2(inner) => {
                println!("VUGen2:");
                println!("{:#?}", inner);
                println!("{:?}", inner.to_json());
            }
        },
        Err(err) => {
            debug!("{:?}", err);
        }
    }
}
