use binary_data::BigEndian;
use serde::Serialize;

use crate::{
    Readable,
    tacho::{CardStructureVersion, EquipmentType},
};

/// Information, stored in a workshop card related to the identification of the
/// application of the card (Annex 1C requirement 307 and 330).
#[derive(Debug, Clone, Serialize)]
pub struct WorkshopCardApplicationIdentification {
    #[serde(rename = "typeOfTachographCardId")]
    pub type_of_tachograph_card_id: EquipmentType,
    #[serde(rename = "cardStructureVersion")]
    pub card_structure_version: CardStructureVersion,
    #[serde(rename = "noOfEventsPerType")]
    pub no_events_per_type: u8,
    #[serde(rename = "noOfFaultsPerType")]
    pub no_faults_per_type: u8,
    #[serde(rename = "activityStructureLength")]
    pub activity_structure_length: u32,
    #[serde(rename = "noOfCardVehicleRecords")]
    pub no_of_card_vehicle_records: u32,
    #[serde(rename = "noOfCardPlaceRecords")]
    pub no_of_card_place_records: u32,
    #[serde(rename = "noOfCalibrationRecords")]
    pub no_off_calibration_records: u8,
    #[serde(rename = "noOfGnssadRecords")]
    pub no_of_gnssad_records: u32,
    #[serde(rename = "noOfSpecificConditionRecords")]
    pub no_of_specific_condition_records: u32,
    #[serde(rename = "noOfCardVehicleUnitRecords")]
    pub no_of_card_vehicle_unit_records: u32,
}

impl Readable<WorkshopCardApplicationIdentification> for WorkshopCardApplicationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<WorkshopCardApplicationIdentification> {
        let type_of_tachograph_card_id = reader.read_u8()?.into();
        let card_structure_version = CardStructureVersion::read(reader)?;
        let no_events_per_type = reader.read_u8()?;
        let no_faults_per_type = reader.read_u8()?;
        let activity_structure_length = reader.read_u16::<BigEndian>()? as u32;
        let no_of_card_vehicle_records = reader.read_u16::<BigEndian>()? as u32;
        let no_of_card_place_records = reader.read_u8()? as u32;
        let no_off_calibration_records = reader.read_u8()?;

        let no_of_gnssad_records = reader.read_u16::<BigEndian>()? as u32;
        let no_of_specific_condition_records = reader.read_u16::<BigEndian>()? as u32;
        let no_of_card_vehicle_unit_records = reader.read_u16::<BigEndian>()? as u32;

        Ok(Self {
            type_of_tachograph_card_id,
            card_structure_version,
            no_events_per_type,
            no_faults_per_type,
            activity_structure_length,
            no_of_card_vehicle_records,
            no_of_card_place_records,
            no_off_calibration_records,
            no_of_gnssad_records,
            no_of_specific_condition_records,
            no_of_card_vehicle_unit_records,
        })
    }
}
