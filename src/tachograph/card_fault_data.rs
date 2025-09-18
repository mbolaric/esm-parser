use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    tacho::{EventFaultType, TimeReal, VehicleRegistrationIdentification},
};

/// Information, stored in a driver or a workshop card, related to a fault
/// associated to the card holder (Annex 1C requirement 264, 289, 318, and 341).
#[derive(Debug, Serialize)]
pub struct CardFaultRecord {
    #[serde(rename = "faultType")]
    pub fault_type: EventFaultType,
    #[serde(rename = "faultBeginTime")]
    pub fault_begin_time: TimeReal,
    #[serde(rename = "faultEndTime")]
    pub fault_end_time: TimeReal,
    #[serde(rename = "faultVehicleRegistration")]
    pub fault_vehicle_registration: VehicleRegistrationIdentification,
}

impl Readable<CardFaultRecord> for CardFaultRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardFaultRecord> {
        let fault_type = reader.read_u8()?.into();
        let fault_begin_time = TimeReal::read(reader)?;
        let fault_end_time = TimeReal::read(reader)?;
        let fault_vehicle_registration = VehicleRegistrationIdentification::read(reader)?;

        Ok(Self { fault_type, fault_begin_time, fault_end_time, fault_vehicle_registration })
    }
}

#[derive(Debug)]
pub struct CardFaultDataParams {
    pub no_faults_per_type: u8,
}

impl CardFaultDataParams {
    pub fn new(no_faults_per_type: u8) -> Self {
        Self { no_faults_per_type }
    }
}

/// Information, stored in a driver or a workshop card, related to the faults
/// associated to the card holder (Annex 1C requirements 263, 288, 318, and 341).
#[derive(Debug, Serialize)]
pub struct CardFaultData {
    #[serde(rename = "noFaultsPerType")]
    pub no_faults_per_type: u8,
    #[serde(rename = "cardFaultRecords")]
    pub card_fault_records: Vec<Vec<CardFaultRecord>>,
}

impl ReadableWithParams<CardFaultData> for CardFaultData {
    type P = CardFaultDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardFaultData> {
        let no_faults_per_type = params.no_faults_per_type;

        let mut card_fault_records: Vec<Vec<CardFaultRecord>> = Vec::new();
        for _ in 0..2 {
            let mut records: Vec<CardFaultRecord> = Vec::new();
            for _ in 0..no_faults_per_type {
                let record = CardFaultRecord::read(reader)?;
                if record.fault_type != EventFaultType::NoFurtherDetails
                    || record.fault_begin_time.data != 0
                    || record.fault_end_time.data != 0
                {
                    records.push(record);
                }
            }
            if !records.is_empty() {
                card_fault_records.push(records);
            }
        }
        Ok(Self { no_faults_per_type, card_fault_records })
    }
}
