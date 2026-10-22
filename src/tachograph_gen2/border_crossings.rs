use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::GnssPlaceAuthRecord;
use crate::tacho::{NationNumeric, OdometerShort};
use crate::{Error, Readable, ReadableWithParams, Result, read_records};

/// Information, stored in a driver or workshop card, related to border crossings
/// (Annex IC requirements 306e and 356e; Appendix 1, section 2.11b).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2BorderCrossingRecord"))]
pub struct BorderCrossingRecord {
    #[serde(rename = "countryLeft")]
    pub country_left: NationNumeric,
    #[serde(rename = "countryEntered")]
    pub country_entered: NationNumeric,
    #[serde(rename = "gnssPlaceAuthRecord")]
    pub gnss_place_auth_record: GnssPlaceAuthRecord,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl Readable for BorderCrossingRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<BorderCrossingRecord> {
        let country_left: NationNumeric = reader.read_u8()?.into();
        let country_entered: NationNumeric = reader.read_u8()?.into();
        let gnss_place_auth_record = GnssPlaceAuthRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        Ok(Self { country_left, country_entered, gnss_place_auth_record, vehicle_odometer_value })
    }
}

#[derive(Debug)]
pub struct BorderCrossingsParams {
    pub no_of_border_crossing_records: u16,
}

impl BorderCrossingsParams {
    pub fn new(no_of_border_crossing_records: u16) -> Self {
        Self { no_of_border_crossing_records }
    }
}

/// Information, stored in a driver or workshop card, related to border crossings
/// (Annex IC requirements 306f and 356f; Appendix 1, section 2.11a).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2BorderCrossings"))]
pub struct BorderCrossings {
    #[serde(rename = "borderCrossingPointerNewestRecord")]
    pub border_crossing_pointer_newest_record: u16,
    #[serde(rename = "cardBorderCrossingRecords")]
    pub card_border_crossing_records: Vec<BorderCrossingRecord>,
}

impl ReadableWithParams for BorderCrossings {
    type P = BorderCrossingsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<BorderCrossings> {
        let count = usize::from(params.no_of_border_crossing_records);
        if count == 0 {
            return Err(Error::RecordOutOfRange("Border crossing record count must be nonzero".to_owned()));
        }
        let expected_size = 2 + count * 17;
        let remaining = reader.len()?.saturating_sub(reader.pos()?);
        if remaining != expected_size {
            return Err(Error::InvalidDataParse(format!(
                "Border crossings EF size: expected {expected_size} bytes for {count} records, got {remaining}"
            )));
        }
        let border_crossing_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        if usize::from(border_crossing_pointer_newest_record) >= count {
            return Err(Error::RecordOutOfRange(format!(
                "Border crossing newest-record pointer: {border_crossing_pointer_newest_record}, capacity: {count}"
            )));
        }
        let records = read_records(reader, count, BorderCrossingRecord::read)?
            .into_iter()
            .filter(|record| {
                let timestamp = &record.gnss_place_auth_record.time_stamp;
                timestamp.has_data() && timestamp.is_set()
            })
            .collect();

        Ok(Self { border_crossing_pointer_newest_record, card_border_crossing_records: records })
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;
    use serde_json::json;

    use super::*;
    use crate::tacho::TimeReal;

    const FIRST: [u8; 17] = [13, 17, 0, 0, 0, 100, 1, 0, 0xB9, 0x94, 0, 0x0D, 0x06, 0, 1, 0xE2, 0x40];
    const SECOND: [u8; 17] = [17, 13, 0, 0, 0, 200, 7, 0, 0x0D, 0x06, 0, 0xB9, 0x94, 1, 9, 0xFB, 0xF1];

    #[test]
    fn container_uses_the_nested_timestamp_and_preserves_the_physical_pointer() {
        let mut second = SECOND;
        second[13] = 0xFE;
        let bytes = [3_u16.to_be_bytes().as_slice(), &[0; 17], FIRST.as_slice(), &[0xFF; 17], second.as_slice()].concat();
        let mut reader = BinMemoryBuffer::from(bytes);
        let records = BorderCrossings::read(&mut reader, &BorderCrossingsParams::new(4)).unwrap();
        assert_eq!(records.border_crossing_pointer_newest_record, 3);
        let value = serde_json::to_value(records).unwrap();
        let records = value["cardBorderCrossingRecords"].as_array().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["gnssPlaceAuthRecord"]["timeStamp"], json!(TimeReal::new(100)));
        assert_eq!(records[1]["gnssPlaceAuthRecord"]["authenticationStatus"], 0xFE);
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    }

    #[test]
    fn container_rejects_invalid_lengths_capacities_and_pointers() {
        let valid = [1_u16.to_be_bytes().as_slice(), FIRST.as_slice(), SECOND.as_slice()].concat();
        for length in [0, 1, 2, 18, 35, 37, 42] {
            let mut bytes = valid.clone();
            bytes.resize(length, 0);
            assert!(BorderCrossings::read(&mut BinMemoryBuffer::from(bytes), &BorderCrossingsParams::new(2)).is_err());
        }
        for pointer in [2_u16, u16::MAX] {
            let mut bytes = valid.clone();
            bytes[..2].copy_from_slice(&pointer.to_be_bytes());
            assert!(BorderCrossings::read(&mut BinMemoryBuffer::from(bytes), &BorderCrossingsParams::new(2)).is_err());
        }
        for count in [0, u16::MAX] {
            assert!(
                BorderCrossings::read(&mut BinMemoryBuffer::from(valid.clone()), &BorderCrossingsParams::new(count)).is_err()
            );
        }
    }
}
