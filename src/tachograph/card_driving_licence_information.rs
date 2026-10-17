use serde::Serialize;

use crate::tacho::{Name, NationNumeric};
use crate::{CodePage, Error, Readable, Validatable, Writable, bytes_to_string, validate_ia5_string, write_ia5_string};

const DRIVING_LICENSE_NUMBER_LENGTH: u32 = 16;

/// Information, stored in a driver card, related to the card holder driver
/// licence data (Annex 1C requirement 259 and 284).
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardDrivingLicenceInformation {
    #[serde(rename = "drivingLicenceIssuingAuthority")]
    pub driving_licence_issuing_authority: Name,
    #[serde(rename = "drivingLicenceIssuingNation")]
    pub driving_licence_issuing_nation: NationNumeric,
    #[serde(rename = "drivingLicenceNumber")]
    pub driving_licence_number: String,
}

impl Validatable for CardDrivingLicenceInformation {
    fn validate(&self) -> crate::Result<()> {
        self.driving_licence_issuing_authority.validate()?;
        if !self.driving_licence_issuing_authority.name.is_empty() && self.driving_licence_number.is_empty() {
            return Err(Error::CorruptedDrivingLicenceNumber);
        }
        validate_ia5_string::<{ DRIVING_LICENSE_NUMBER_LENGTH as usize }>(
            "CardDrivingLicenceInformation.driving_licence_number",
            &self.driving_licence_number,
        )
    }
}

impl Readable for CardDrivingLicenceInformation {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardDrivingLicenceInformation> {
        let driving_licence_issuing_authority = Name::read(reader)?;
        let driving_licence_issuing_nation: NationNumeric = reader.read_u8()?.into();
        let driving_licence_number =
            bytes_to_string(&reader.read_into_vec(DRIVING_LICENSE_NUMBER_LENGTH)?, &CodePage::IsoIec8859_1);
        if !driving_licence_issuing_authority.name.is_empty() && driving_licence_number.is_empty() {
            return Err(Error::CorruptedDrivingLicenceNumber);
        }

        Ok(Self { driving_licence_issuing_authority, driving_licence_issuing_nation, driving_licence_number })
    }
}

impl Writable for CardDrivingLicenceInformation {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        self.driving_licence_issuing_authority.write_raw(writer)?;
        writer.write_u8(self.driving_licence_issuing_nation as u8)?;
        write_ia5_string::<{ DRIVING_LICENSE_NUMBER_LENGTH as usize }, _>(
            writer,
            "CardDrivingLicenceInformation.driving_licence_number",
            &self.driving_licence_number,
        )
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

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
