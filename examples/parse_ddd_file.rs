mod helpers;

use esm_parser::EsmParser;
use helpers::init_logging;
use log::debug;

fn main() {
    init_logging();

    match EsmParser::parse("./examples/data/TestTachoDataGen2001.DDD") {
        Ok(parser) => {
            debug!("{:?}", parser);
        }
        Err(err) => {
            debug!("{:?}", err);
        }
    }
}
