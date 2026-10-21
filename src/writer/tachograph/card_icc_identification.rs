use crate::tacho::CardIccIdentification;
use crate::writer::{Validatable, Writable, validate_fixed_bytes, validate_ia5_string, write_ia5_string};

impl Validatable for CardIccIdentification {
    fn validate(&self) -> crate::Result<()> {
        self.card_extended_serial_number.validate()?;
        validate_ia5_string::<{ CardIccIdentification::CARD_APPROVAL_NUMBER_LENGTH as usize }>(
            "CardIccIdentification.card_approval_number",
            &self.card_approval_number,
        )?;
        self.embedder_ic_assembler_id.validate()?;
        validate_fixed_bytes::<{ CardIccIdentification::IC_IDENTIFIER_NUMBER_LENGTH as usize }>(
            "CardIccIdentification.ic_identifier",
            &self.ic_identifier,
        )
    }
}

impl Writable for CardIccIdentification {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.clock_stop)?;
        self.card_extended_serial_number.write_raw(writer)?;
        write_ia5_string::<{ CardIccIdentification::CARD_APPROVAL_NUMBER_LENGTH as usize }, _>(
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

    use crate::Readable;
    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};

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
