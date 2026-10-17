use serde::Serialize;

use crate::tacho::{Datef, HolderName};
use crate::{Readable, Validatable, Writable, bytes_to_ia5_fix_string, validate_ia5_string, write_ia5_string};

const CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH: u32 = 2;

/// Information, stored in a driver card, related to the identification of the
/// cardholder (Annex 1C requirement 256 and 281).
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct DriverCardHolderIdentification {
    #[serde(rename = "cardHolderName")]
    pub card_holder_name: HolderName,
    #[serde(rename = "cardHolderBirthDate")]
    pub card_holder_birth_date: Datef,
    #[serde(rename = "cardHolderPreferredLanguage")]
    pub card_holder_preferred_language: String,
}

impl Validatable for DriverCardHolderIdentification {
    fn validate(&self) -> crate::Result<()> {
        self.card_holder_name.validate()?;
        self.card_holder_birth_date.validate()?;
        validate_ia5_string::<{ CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH as usize }>(
            "DriverCardHolderIdentification.card_holder_preferred_language",
            &self.card_holder_preferred_language,
        )
    }
}

impl Readable for DriverCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<DriverCardHolderIdentification> {
        let card_holder_name = HolderName::read(reader)?;
        let card_holder_birth_date = Datef::read(reader)?;
        let card_holder_preferred_language =
            bytes_to_ia5_fix_string(&reader.read_into_vec(CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH)?)?;

        Ok(Self { card_holder_name, card_holder_birth_date, card_holder_preferred_language })
    }
}

impl Writable for DriverCardHolderIdentification {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        self.card_holder_name.write_raw(writer)?;
        self.card_holder_birth_date.write_raw(writer)?;
        write_ia5_string::<{ CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH as usize }, _>(
            writer,
            "DriverCardHolderIdentification.card_holder_preferred_language",
            &self.card_holder_preferred_language,
        )
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;
    use crate::CodePage;
    use crate::tacho::Name;

    #[test]
    fn test_driver_card_holder_identification_roundtrip() {
        // HolderName: 36 bytes surname + 36 bytes first name = 72 bytes
        let mut data = vec![0x01];
        let mut surname = b"Mustermann".to_vec();
        surname.resize(35, 0x20);
        data.extend_from_slice(&surname);
        data.push(0x01);
        let mut first = b"Max".to_vec();
        first.resize(35, 0x20);
        data.extend_from_slice(&first);

        // Datef: 2 bytes year BCD + 1 byte month BCD + 1 byte day BCD = 4 bytes
        data.extend_from_slice(&[0x19, 0x85, 0x05, 0x20]);

        // Language: 2 bytes
        data.extend_from_slice(b"de");

        let mut reader = BinMemoryBuffer::from(data.clone());
        let info = DriverCardHolderIdentification::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        info.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_driver_card_holder_identification_validation_and_zero_byte_emission() {
        let invalid_lang = DriverCardHolderIdentification {
            card_holder_name: HolderName {
                holder_surname: Name { code_page: CodePage::IsoIec8859_1, name: "Mustermann".into() },
                holder_first_names: Name { code_page: CodePage::IsoIec8859_1, name: "Max".into() },
            },
            card_holder_birth_date: Datef { year: "1985".into(), month: "05".into(), day: "20".into() },
            card_holder_preferred_language: "deu".into(), // > 2 bytes
        };
        assert!(invalid_lang.validate().is_err());

        // Even though name and birth date are valid, 0 bytes must be written when language fails
        let mut writer = BinMemoryBuffer::new();
        assert!(invalid_lang.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
