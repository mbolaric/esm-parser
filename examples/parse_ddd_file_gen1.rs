mod helpers;

use esm_parser::EsmParser;
use helpers::init_logging;
use log::debug;

fn main() {
    init_logging();

    // TestTachoData.DDD
    match EsmParser::parse("./examples/data/C_20190701_1042_K_Musterfrau 10_11000000071640.DDD") {
        Ok(parser) => {
            debug!("{:?}", parser);
        }
        Err(err) => {
            debug!("{:?}", err);
        }
    }
}
