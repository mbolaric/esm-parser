use crate::Error;
use crate::tacho::CardDrivingLicenceInformation;
use crate::writer::{Validatable, Writable, validate_ia5_string, write_ia5_string};

impl Validatable for CardDrivingLicenceInformation {
    fn validate(&self) -> crate::Result<()> {
        self.driving_licence_issuing_authority.validate()?;
        if !self.driving_licence_issuing_authority.name.is_empty() && self.driving_licence_number.is_empty() {
            return Err(Error::CorruptedDrivingLicenceNumber);
        }
        validate_ia5_string::<{ CardDrivingLicenceInformation::DRIVING_LICENSE_NUMBER_LENGTH as usize }>(
            "CardDrivingLicenceInformation.driving_licence_number",
            &self.driving_licence_number,
        )
    }
}

impl Writable for CardDrivingLicenceInformation {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        self.driving_licence_issuing_authority.write_raw(writer)?;
        writer.write_u8(self.driving_licence_issuing_nation as u8)?;
        write_ia5_string::<{ CardDrivingLicenceInformation::DRIVING_LICENSE_NUMBER_LENGTH as usize }, _>(
            writer,
            "CardDrivingLicenceInformation.driving_licence_number",
            &self.driving_licence_number,
        )
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};
    use crate::{CodePage, Readable};

    #[test]
    fn test_card_driving_licence_information_roundtrip() {
        // Name: 1 byte codepage + 35 bytes
        let mut data = vec![0x01];
        let mut auth = b"Auth".to_vec();
        auth.resize(35, 0x20);
        data.extend_from_slice(&auth);
        // NationNumeric: 1 byte
        data.push(0x01);
        // Driving licence number: 16 bytes
        let mut dln = b"DL12345678".to_vec();
        dln.resize(16, 0x20);
        data.extend_from_slice(&dln);

        let mut reader = BinMemoryBuffer::from(data.clone());
        let info = CardDrivingLicenceInformation::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        info.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_card_driving_licence_information_validation_and_zero_byte_emission() {
        // Overlong driving licence number (> 16 bytes)
        let overlong = CardDrivingLicenceInformation {
            driving_licence_issuing_authority: Name { code_page: CodePage::IsoIec8859_1, name: "Auth".into() },
            driving_licence_issuing_nation: NationNumeric::Austria,
            driving_licence_number: "A".repeat(17),
        };
        assert!(overlong.validate().is_err());
        let mut writer = BinMemoryBuffer::new();
        assert!(overlong.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);

        // Corrupted: non-empty authority with empty licence number
        let corrupted = CardDrivingLicenceInformation {
            driving_licence_issuing_authority: Name { code_page: CodePage::IsoIec8859_1, name: "Auth".into() },
            driving_licence_issuing_nation: NationNumeric::Austria,
            driving_licence_number: String::new(),
        };
        assert!(corrupted.validate().is_err());
        let mut writer2 = BinMemoryBuffer::new();
        assert!(corrupted.write(&mut writer2).is_err());
        assert_eq!(writer2.as_slice().len(), 0);
    }
}
