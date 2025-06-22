mod helpers;

use esm::EsmParser;
use helpers::init_logging;
use log::debug;

fn main() {
    init_logging();

    match EsmParser::parse("./examples/data/TestTachoData.DDD") {
        Ok(parser) => {
            debug!("{:?}", parser);
        }
        Err(err) => {
            debug!("{:?}", err);
        }
    }
}
