use crate::tacho::VehicleRegistrationNumber;
use crate::writer::{Validatable, Writable, validate_code_page_string, write_code_page_string};

impl Validatable for VehicleRegistrationNumber {
    /// Validates the vehicle registration number against tachograph specifications:
    /// - Code page must be valid and supported.
    /// - All characters must be representable without loss in the specified code page.
    /// - Encoded byte length must not exceed 13 bytes.
    fn validate(&self) -> crate::Result<()> {
        validate_code_page_string::<{ VehicleRegistrationNumber::VEHICLE_REG_NUMBER_LENGTH as usize }>(
            "VehicleRegistrationNumber",
            &self.vehicle_reg_number,
            &self.code_page,
        )
    }
}

impl Writable for VehicleRegistrationNumber {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        write_code_page_string::<{ VehicleRegistrationNumber::VEHICLE_REG_NUMBER_LENGTH as usize }, _>(
            writer,
            "VehicleRegistrationNumber",
            &self.code_page,
            &self.vehicle_reg_number,
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
    fn test_vehicle_registration_number_roundtrip() {
        let mut data = vec![0x01]; // CodePage::IsoIec8859_1
        let vrn = "TEST-VRN";
        let mut vrn_bytes = vrn.as_bytes().to_vec();
        vrn_bytes.resize(13, 0x20);
        data.extend_from_slice(&vrn_bytes);

        let mut reader = BinMemoryBuffer::from(data.clone());
        let vrn_struct = VehicleRegistrationNumber::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        vrn_struct.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_vehicle_registration_number_validation_and_zero_byte_emission() {
        let exact_13 = VehicleRegistrationNumber { code_page: CodePage::IsoIec8859_1, vehicle_reg_number: "A".repeat(13) };
        assert!(exact_13.validate().is_ok());

        let overlong = VehicleRegistrationNumber { code_page: CodePage::IsoIec8859_1, vehicle_reg_number: "A".repeat(14) };
        assert!(overlong.validate().is_err());
        let mut buffer = BinMemoryBuffer::new();
        assert!(overlong.write(&mut buffer).is_err());
        assert_eq!(buffer.as_slice().len(), 0);

        let invalid_cp = VehicleRegistrationNumber { code_page: CodePage::Invalid, vehicle_reg_number: "TEST-1".into() };
        assert!(invalid_cp.validate().is_err());
        let mut buffer2 = BinMemoryBuffer::new();
        assert!(invalid_cp.write(&mut buffer2).is_err());
        assert_eq!(buffer2.as_slice().len(), 0);

        let unencodable = VehicleRegistrationNumber { code_page: CodePage::IsoIec8859_1, vehicle_reg_number: "VRN-€".into() };
        assert!(unencodable.validate().is_err());
        let mut buffer3 = BinMemoryBuffer::new();
        assert!(unencodable.write(&mut buffer3).is_err());
        assert_eq!(buffer3.as_slice().len(), 0);
    }
}
