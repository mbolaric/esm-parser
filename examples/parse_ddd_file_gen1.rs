mod helpers;

use esm_parser::Export;
use esm_parser::{TachographData, parse_from_file};
use helpers::init_logging;
use log::debug;

fn main() {
    init_logging();

    match parse_from_file("./examples/data/TestTachoData.DDD") {
        Ok(data) => match data {
            TachographData::CardGen1(inner) => {
                println!("CardGen1:");
                println!("{:#?}", inner);
                println!("{:?}", inner.to_json());
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
