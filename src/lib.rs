mod common;
mod consts;
mod error;
mod helpers;
mod parser;
mod tachograph;
mod tachograph_data;
mod tachograph_gen1;
mod tachograph_gen2;

pub use common::*;
pub use consts::*;
pub use error::{Error, Result};
pub mod tacho {
    pub use super::tachograph::*;
}
pub mod gen1 {
    pub use super::tachograph_gen1::*;
}
pub mod gen2 {
    pub use super::tachograph_gen2::*;
}
pub use parser::{parse_from_file, parse_from_memory};
pub use tachograph_data::TachographData;
