use crate::tacho::DriverCardHolderIdentification;
use crate::writer::{Validatable, Writable, validate_ia5_string, write_ia5_string};

impl Validatable for DriverCardHolderIdentification {
    fn validate(&self) -> crate::Result<()> {
        self.card_holder_name.validate()?;
        self.card_holder_birth_date.validate()?;
        validate_ia5_string::<{ DriverCardHolderIdentification::CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH as usize }>(
            "DriverCardHolderIdentification.card_holder_preferred_language",
            &self.card_holder_preferred_language,
        )
    }
}

impl Writable for DriverCardHolderIdentification {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        self.card_holder_name.write_raw(writer)?;
        self.card_holder_birth_date.write_raw(writer)?;
        write_ia5_string::<{ DriverCardHolderIdentification::CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH as usize }, _>(
            writer,
            "DriverCardHolderIdentification.card_holder_preferred_language",
            &self.card_holder_preferred_language,
        )
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::tacho::{Name, *};
    use crate::writer::{Validatable, Writable};
    use crate::{CodePage, Readable};

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
