use std::collections::HashMap;

use crate::tacho::{CardDataFile, CardFileID};
use crate::{Error, Result};

#[derive(Debug)]
pub struct ControlCard {}

impl ControlCard {
    pub fn parse(_card_data_files: &HashMap<CardFileID, CardDataFile>, _card_notes: &str) -> Result<Box<ControlCard>> {
        Err(Error::NotImplemented)
    }
}
