use binary_data::BigEndian;
use serde::Serialize;

use crate::tacho::{CardStructureVersion, EquipmentType};
use crate::{Error, Readable, Result};

/// Information, stored in a workshop card related to the identification of the
/// application of the card (Annex 1C requirement 307 and 330).
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2WorkshopCardApplicationIdentification"))]
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
    pub no_off_calibration_records: u16,
    #[serde(rename = "noOfGnssadRecords")]
    pub no_of_gnssad_records: u32,
    #[serde(rename = "noOfSpecificConditionRecords")]
    pub no_of_specific_condition_records: u32,
    #[serde(rename = "noOfCardVehicleUnitRecords")]
    pub no_of_card_vehicle_unit_records: u32,
}

impl Readable for WorkshopCardApplicationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> Result<WorkshopCardApplicationIdentification> {
        let type_of_tachograph_card_id = reader.read_u8()?.into();
        let card_structure_version = CardStructureVersion::read(reader)?;
        let no_events_per_type = reader.read_u8()?;
        let no_faults_per_type = reader.read_u8()?;
        let activity_structure_length = reader.read_u16::<BigEndian>()? as u32;
        let no_of_card_vehicle_records = reader.read_u16::<BigEndian>()? as u32;
        let no_of_card_place_records = reader.read_u16::<BigEndian>()? as u32;
        let no_off_calibration_records = reader.read_u16::<BigEndian>()?;

        let no_of_gnssad_records = reader.read_u16::<BigEndian>()? as u32;
        let no_of_specific_condition_records = reader.read_u16::<BigEndian>()? as u32;
        if no_of_specific_condition_records > 4 {
            return Err(Error::RecordOutOfRange(format!(
                "Workshop specific-condition capacity: {no_of_specific_condition_records}"
            )));
        }
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

#[cfg(test)]
mod tests {
    use binary_data::{BinMemoryBuffer, BinSeek};

    use super::*;

    #[test]
    fn reads_all_gen2_workshop_capacities_at_their_wire_widths() {
        let mut bytes = vec![2, 1, 1, 3, 6];
        for capacity in [492_u16, 8, 8, 255, 24, 4, 8] {
            bytes.extend(capacity.to_be_bytes());
        }
        let mut reader = BinMemoryBuffer::from(bytes);
        let app = WorkshopCardApplicationIdentification::read(&mut reader).unwrap();
        assert_eq!(reader.pos().unwrap(), 19);
        assert_eq!(app.activity_structure_length, 492);
        assert_eq!(app.no_of_card_vehicle_records, 8);
        assert_eq!(app.no_of_card_place_records, 8);
        assert_eq!(app.no_off_calibration_records, 255);
        assert_eq!(app.no_of_gnssad_records, 24);
        assert_eq!(app.no_of_specific_condition_records, 4);
        assert_eq!(app.no_of_card_vehicle_unit_records, 8);
    }
}
