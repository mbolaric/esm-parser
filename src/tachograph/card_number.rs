use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::EquipmentType;
use crate::{ReadableWithParams, Result, bytes_to_ia5_fix_string};

#[derive(Debug)]
pub struct CardNumberParams {
    pub equipment_type: EquipmentType,
}

impl CardNumberParams {
    pub fn new(equipment_type: EquipmentType) -> Self {
        Self { equipment_type }
    }
}

/// A card number as defined by definition g).
/// CardNumber ::~ CHOICE {
///     SEQUENCE {
///         driveridentification    IA5String(SIZE(14)),
///         cardReplacementindex    CardReplacementindex,
///         cardRenewalindex        CardRenewalindex
///     },
///     SEQUENCE {
///         owneridentification     IA5String(SIZE(13)),
///         cardConsecutiveindex    CardConsecutiveindex,
///         cardReplacementindex    CardReplacementindex,
///         cardRenewalindex        CardRenewalindex
///     }
/// }
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardNumber {
    #[serde(rename = "equipmentType")]
    pub equipment_type: EquipmentType,
    pub number: String,
    pub identification: String,
    #[serde(rename = "cardReplacementindex")]
    pub card_replacement_index: String,
    #[serde(rename = "cardConsecutiveindex")]
    pub card_consecutive_index: String,
    #[serde(rename = "cardRenewalindex")]
    pub card_renewal_index: String,
}

impl CardNumber {
    pub(crate) const CARD_NUMBER_LENGTH: u32 = 16;
}

impl ReadableWithParams for CardNumber {
    type P = CardNumberParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardNumber> {
        let equipment_type = params.equipment_type;
        let data = reader.read_bytes::<{ CardNumber::CARD_NUMBER_LENGTH as usize }>()?;
        let number = bytes_to_ia5_fix_string(&data)?;
        // Split physical positions before trimming padding from each component.
        let identification_length = match equipment_type {
            EquipmentType::DriverCard => 14,
            EquipmentType::WorkshopCard | EquipmentType::ControlCard | EquipmentType::CompanyCard => 13,
            _ => {
                return Ok(Self {
                    number,
                    equipment_type,
                    identification: String::new(),
                    card_replacement_index: String::new(),
                    card_consecutive_index: String::new(),
                    card_renewal_index: String::new(),
                });
            }
        };
        let identification = bytes_to_ia5_fix_string(&data[..identification_length])?;
        let card_consecutive_index =
            if equipment_type == EquipmentType::DriverCard { String::new() } else { bytes_to_ia5_fix_string(&data[13..14])? };
        let card_replacement_index = bytes_to_ia5_fix_string(&data[14..15])?;
        let card_renewal_index = bytes_to_ia5_fix_string(&data[15..16])?;
        Ok(Self { number, equipment_type, identification, card_replacement_index, card_consecutive_index, card_renewal_index })
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;
    use crate::Export;

    #[test]
    fn test_read_driver_card_number() {
        let data: &[u8] = b"ABC1234567890123";
        let mut reader = BinMemoryBuffer::from(data.to_vec());
        let params = CardNumberParams::new(EquipmentType::DriverCard);
        let card_number = CardNumber::read(&mut reader, &params).unwrap();

        assert_eq!(card_number.number, "ABC1234567890123");
        assert_eq!(card_number.identification, "ABC12345678901");
        assert_eq!(card_number.card_consecutive_index, "");
        assert_eq!(card_number.card_replacement_index, "2");
        assert_eq!(card_number.card_renewal_index, "3");
    }

    #[test]
    fn test_read_workshop_card_number() {
        let data: &[u8] = b"WSP9876543210987";
        let mut reader = BinMemoryBuffer::from(data.to_vec());
        let params = CardNumberParams::new(EquipmentType::WorkshopCard);
        let card_number = CardNumber::read(&mut reader, &params).unwrap();

        assert_eq!(card_number.number, "WSP9876543210987");
        assert_eq!(card_number.identification, "WSP9876543210");
        assert_eq!(card_number.card_consecutive_index, "9");
        assert_eq!(card_number.card_replacement_index, "8");
        assert_eq!(card_number.card_renewal_index, "7");
    }

    #[test]
    fn test_export_card_number_uses_equipment_type() {
        for equipment_type in
            [EquipmentType::DriverCard, EquipmentType::WorkshopCard, EquipmentType::ControlCard, EquipmentType::CompanyCard]
        {
            let mut reader = BinMemoryBuffer::from(b"ABC1234567890123".to_vec());
            let card_number = CardNumber::read(&mut reader, &CardNumberParams::new(equipment_type)).unwrap();

            let json: serde_json::Value = serde_json::from_str(&card_number.to_json().unwrap()).unwrap();
            assert_eq!(json["equipmentType"], serde_json::to_value(equipment_type).unwrap());
            assert!(json.get("cardIssuingMemberState").is_none());
            assert_eq!(json["number"], "ABC1234567890123");

            let xml = card_number.to_xml().unwrap();
            assert!(xml.contains(&format!("<equipmentType>{equipment_type}</equipmentType>")));
            assert!(!xml.contains("cardIssuingMemberState"));
        }
    }

    #[test]
    fn test_read_short_card_number() {
        let data: &[u8] = b"SHORT";
        let mut reader = BinMemoryBuffer::from(data.to_vec());
        let params = CardNumberParams::new(EquipmentType::DriverCard);
        let result = CardNumber::read(&mut reader, &params);
        assert!(result.is_err());
    }
}
