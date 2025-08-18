use std::collections::HashMap;

use crate::tacho::{CardFileData, CardFileID};
use crate::{Error, Result};

#[derive(Debug)]
pub struct ControlCard {}

impl ControlCard {
    pub fn parse(_card_data_files: &HashMap<CardFileID, CardFileData>, _card_notes: &str) -> Result<Box<ControlCard>> {
        // Card Certificate
        // CertificateApplicationIdentification '0501h'
        // Identification '0520h'
        // ControllerActivityData '050Ch'

        Err(Error::NotImplemented)
    }
}
