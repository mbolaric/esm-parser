mod helpers;

use esm_parser::{EsmParser, TachographData};
use helpers::init_logging;
use log::debug;

fn main() {
    init_logging();

    match EsmParser::parse("./examples/data/TypeCard.DDD") {
        Ok(parser) => {
            if let Some(data) = parser.get_data() {
                match data {
                    TachographData::CardGen1(inner) => {
                        println!("CardGen1:");
                        println!("{:#?}", inner);
                    }
                    TachographData::CardGen2(inner) => {
                        println!("CardGen2:");
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
