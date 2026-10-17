use serde::Serialize;

use crate::tacho::{EmbedderIcAssemblerId, ExtendedSerialNumber};
use crate::{
    CodePage, Readable, Validatable, Writable, bytes_to_string, validate_fixed_bytes, validate_ia5_string, write_ia5_string,
};

const CARD_APPROVAL_NUMBER_LENGTH: u32 = 8;
const IC_IDENTIFIER_NUMBER_LENGTH: u32 = 2;

/// Information, stored in a card, related to the identification of the integrated
/// circuit (IC) card (Annex 1C requirement 248).
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardIccIdentification {
    #[serde(rename = "clockStop")]
    pub clock_stop: u8,
    #[serde(rename = "cardExtendedSerialNumber")]
    pub card_extended_serial_number: ExtendedSerialNumber,
    #[serde(rename = "cardApprovalNumber")]
    pub card_approval_number: String,
    #[serde(rename = "cardPersonaliserID")]
    pub card_personaliser_id: u8,
    #[serde(rename = "embedderIcAssemblerId")]
    pub embedder_ic_assembler_id: EmbedderIcAssemblerId,
    #[serde(rename = "icIdentifier")]
    pub ic_identifier: Vec<u8>,
}

impl Validatable for CardIccIdentification {
    fn validate(&self) -> crate::Result<()> {
        self.card_extended_serial_number.validate()?;
        validate_ia5_string::<{ CARD_APPROVAL_NUMBER_LENGTH as usize }>(
            "CardIccIdentification.card_approval_number",
            &self.card_approval_number,
        )?;
        self.embedder_ic_assembler_id.validate()?;
        validate_fixed_bytes::<{ IC_IDENTIFIER_NUMBER_LENGTH as usize }>(
            "CardIccIdentification.ic_identifier",
            &self.ic_identifier,
        )
    }
}

impl Readable for CardIccIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardIccIdentification> {
        let clock_stop = reader.read_u8()?;
        let card_extended_serial_number = ExtendedSerialNumber::read(reader)?;
        // It's a string encoded using IA5, which corresponds to ASCII characters.
        let card_approval_number = bytes_to_string(&reader.read_into_vec(CARD_APPROVAL_NUMBER_LENGTH)?, &CodePage::IsoIec8859_1);
        let card_personaliser_id = reader.read_u8()?;
        let embedder_ic_assembler_id = EmbedderIcAssemblerId::read(reader)?;
        // OCTET STRING(SIZE(l))
        // You should interpret it as two raw bytes that represent a Identifier of the IC on the card (not a printable character). So:
        //  - Do not decode it as ASCII or UTF-8
        //  - Instead, treat it like a numeric or binary ID
        //  - usually interpreted as a hex code
        let ic_identifier = reader.read_into_vec(IC_IDENTIFIER_NUMBER_LENGTH)?;

        Ok(Self {
            clock_stop,
            card_extended_serial_number,
            card_approval_number,
            card_personaliser_id,
            embedder_ic_assembler_id,
            ic_identifier,
        })
    }
}

impl Writable for CardIccIdentification {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.clock_stop)?;
        self.card_extended_serial_number.write_raw(writer)?;
        write_ia5_string::<{ CARD_APPROVAL_NUMBER_LENGTH as usize }, _>(
            writer,
            "CardIccIdentification.card_approval_number",
            &self.card_approval_number,
        )?;
        writer.write_u8(self.card_personaliser_id)?;
        self.embedder_ic_assembler_id.write_raw(writer)?;
        writer.write_all(&self.ic_identifier)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn test_card_icc_identification_roundtrip() {
        let mut data = vec![0x00]; // clock_stop
        // ExtendedSerialNumber: 4 bytes serial, 2 bytes month_year BCD, 1 byte type, 1 byte manufacturer
        data.extend_from_slice(&[0x00, 0x01, 0x02, 0x03, 0x06, 0x24, 0x01, 0x02]);
        // cardApprovalNumber: 8 bytes
        data.extend_from_slice(b"E1234   ");
        // cardPersonaliserID: 1 byte
        data.push(0x55);
        // embedderIcAssemblerId: 2 bytes country "DE", 1 byte module BCD 0x12, 2 bytes manufacturer
        data.extend_from_slice(&[b'D', b'E', 0x12, 0x01, 0x02]);
        // icIdentifier: 2 bytes
        data.extend_from_slice(&[0xAA, 0xBB]);

        let mut reader = BinMemoryBuffer::from(data.clone());
        let icc = CardIccIdentification::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        icc.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_card_icc_identification_validation_and_zero_byte_emission() {
        let invalid = CardIccIdentification {
            clock_stop: 0,
            card_extended_serial_number: ExtendedSerialNumber {
                serial_number: 1,
                month_year: "0624".into(),
                serial_type: 1,
                manufacturer_code: 2,
            },
            card_approval_number: "OVERLONG123".into(), // > 8 bytes
            card_personaliser_id: 1,
            embedder_ic_assembler_id: EmbedderIcAssemblerId {
                country_code: "DE".into(),
                module_embedder: "12".into(),
                manufacturer_information: vec![0x01, 0x02],
            },
            ic_identifier: vec![0xAA, 0xBB],
        };
        assert!(invalid.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(invalid.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
