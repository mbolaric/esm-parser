mod helpers;

use esm_parser::{EsmParser, TachographData};
use helpers::init_logging;
use log::debug;

fn main() {
    init_logging();

    // TestTachoData.DDD
    match EsmParser::parse("./examples/data/C_20190701_1042_K_Musterfrau 10_11000000071640.DDD") {
        Ok(parser) => {
            if let Some(data) = parser.get_data() {
                match data {
                    TachographData::CardGen1(inner) => {
                        println!("CardGen1:");
                        println!("{:#?}", inner);
                    }
                    TachographData::VUGen1(inner) => {
                        println!("VUGen1:");
                        println!("{:#?}", inner);
                    }
                    TachographData::VUGen2(inner) => {
                        println!("VUGen2:");
                        println!("{:#?}", inner);
                    }
                }
            } else {
                debug!("{:#?}", parser);
            }
        }
        Err(err) => {
            debug!("{:?}", err);
        }
    }
}
