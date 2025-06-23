use std::fmt;

use crate::{gen1, gen2};

#[derive(Debug)]
pub enum TachographData {
    VUGen1(gen1::VUData),
    VUGen2(gen2::VUData),
    CardGen1(gen1::CardData),
}

impl fmt::Display for TachographData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
