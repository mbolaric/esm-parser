use crate::tacho::VehicleRegistrationIdentification;
use crate::writer::{Validatable, Writable};

impl Validatable for VehicleRegistrationIdentification {
    fn validate(&self) -> crate::Result<()> {
        self.vehicle_registration_number.validate()
    }
}

impl Writable for VehicleRegistrationIdentification {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.vehicle_registration_nation as u8)?;
        self.vehicle_registration_number.write_raw(writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};
    use crate::{CodePage, Readable};

    #[test]
    fn test_vehicle_registration_identification_roundtrip() {
        let mut data = vec![0x01]; // NationNumeric
        data.push(0x01); // CodePage::IsoIec8859_1
        let vrn = "TEST-123";
        let mut vrn_bytes = vrn.as_bytes().to_vec();
        vrn_bytes.resize(13, 0x20);
        data.extend_from_slice(&vrn_bytes);

        let mut reader = BinMemoryBuffer::from(data.clone());
        let parsed = VehicleRegistrationIdentification::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        parsed.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_vehicle_registration_identification_validation_and_zero_byte_emission() {
        let invalid = VehicleRegistrationIdentification {
            vehicle_registration_nation: NationNumeric::Austria,
            vehicle_registration_number: VehicleRegistrationNumber {
                code_page: CodePage::IsoIec8859_1,
                vehicle_reg_number: "OVERLONG-VRN-123".into(),
            },
        };
        assert!(invalid.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(invalid.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
