use serde::Serialize;

use crate::{HexDisplay, Readable, Validatable, Writable, validate_fixed_bytes};

/// Information, stored in a card, related to the identification of the card's
/// Integrated Circuit (IC) (Annex 1C requirement 249). The icSerialNumber
/// together with the icManufacturingReferences identifies the card chip
/// uniquely. The icSerialNumber alone does not uniquely identify the card chip.
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardChipIdentification {
    #[serde(rename = "icSerialNumber")]
    pub ic_serial_number: Vec<u8>,
    #[serde(rename = "icManufacturingReferences")]
    pub ic_manufacturing_references: Vec<u8>,
    #[serde(rename = "icSerialNumberHex")]
    pub ic_serial_number_hex: String,
    #[serde(rename = "icManufacturingReferencesHex")]
    pub ic_manufacturing_references_hex: String,
}

impl Validatable for CardChipIdentification {
    fn validate(&self) -> crate::Result<()> {
        validate_fixed_bytes::<4>("CardChipIdentification.ic_serial_number", &self.ic_serial_number)?;
        validate_fixed_bytes::<4>("CardChipIdentification.ic_manufacturing_references", &self.ic_manufacturing_references)
    }
}

impl Readable for CardChipIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardChipIdentification> {
        let ic_serial_number = reader.read_into_vec(4)?;
        let ic_manufacturing_references = reader.read_into_vec(4)?;

        let ic_serial_number_hex = ic_serial_number.to_hex_string();
        let ic_manufacturing_references_hex = ic_manufacturing_references.to_hex_string();

        Ok(Self { ic_serial_number, ic_manufacturing_references, ic_serial_number_hex, ic_manufacturing_references_hex })
    }
}

impl Writable for CardChipIdentification {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_all(&self.ic_serial_number)?;
        writer.write_all(&self.ic_manufacturing_references)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn test_card_chip_identification_roundtrip() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = BinMemoryBuffer::from(data.clone());
        let chip = CardChipIdentification::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        chip.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_card_chip_identification_validation_and_zero_byte_emission() {
        let invalid = CardChipIdentification {
            ic_serial_number: vec![1, 2, 3, 4],
            ic_manufacturing_references: vec![5, 6, 7], // 3 bytes instead of 4
            ic_serial_number_hex: String::new(),
            ic_manufacturing_references_hex: String::new(),
        };
        assert!(invalid.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(invalid.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
