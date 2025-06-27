use std::collections::HashMap;

use crate::tacho::{CardDataFile, CardFileID};
use crate::{Error, Result};

#[derive(Debug)]
pub struct WorkshopCard {}

impl WorkshopCard {
    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<Box<WorkshopCard>> {
        Err(Error::NotImplemented)
    }
}
