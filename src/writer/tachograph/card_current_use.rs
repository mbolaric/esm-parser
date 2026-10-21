use crate::tacho::CardCurrentUse;
use crate::writer::{Validatable, Writable};

impl Validatable for CardCurrentUse {
    fn validate(&self) -> crate::Result<()> {
        self.session_open_time.validate()?;
        self.session_open_vehicle.validate()?;
        Ok(())
    }
}

impl Writable for CardCurrentUse {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        self.session_open_time.write_raw(writer)?;
        self.session_open_vehicle.write_raw(writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::tacho::{NationNumeric, VehicleRegistrationNumber, *};
    use crate::writer::{Validatable, Writable};
    use crate::{CodePage, Readable};

    #[test]
    fn test_card_current_use_roundtrip() {
        let mut data = vec![0x00, 0x00, 0x01, 0x00]; // TimeReal 256
        data.push(0x01); // NationNumeric
        data.push(0x01); // CodePage::IsoIec8859_1
        let vrn = "TEST-123";
        let mut vrn_bytes = vrn.as_bytes().to_vec();
        vrn_bytes.resize(13, 0x20);
        data.extend_from_slice(&vrn_bytes);

        let mut reader = BinMemoryBuffer::from(data.clone());
        let cur = CardCurrentUse::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        cur.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_card_current_use_validation_and_zero_byte_emission() {
        let invalid = CardCurrentUse {
            session_open_time: TimeReal::new(256),
            session_open_vehicle: VehicleRegistrationIdentification {
                vehicle_registration_nation: NationNumeric::Austria,
                vehicle_registration_number: VehicleRegistrationNumber {
                    code_page: CodePage::Invalid,
                    vehicle_reg_number: "TEST".into(),
                },
            },
        };
        assert!(invalid.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(invalid.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
