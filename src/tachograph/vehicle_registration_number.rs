use serde::Serialize;

use crate::{CodePage, Readable, Validatable, Writable, bytes_to_string, validate_code_page_string, write_code_page_string};

const VEHICLE_REG_NUMBER_LENGTH: u32 = 13;

/// Registration number of the vehicle (VRN). The registration number is
/// assigned by the vehicle licensing authority.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(type = "string"))]
pub struct VehicleRegistrationNumber {
    pub code_page: CodePage,
    pub vehicle_reg_number: String,
}

impl Validatable for VehicleRegistrationNumber {
    /// Validates the vehicle registration number against tachograph specifications:
    /// - Code page must be valid and supported.
    /// - All characters must be representable without loss in the specified code page.
    /// - Encoded byte length must not exceed 13 bytes.
    fn validate(&self) -> crate::Result<()> {
        validate_code_page_string::<{ VEHICLE_REG_NUMBER_LENGTH as usize }>(
            "VehicleRegistrationNumber",
            &self.vehicle_reg_number,
            &self.code_page,
        )
    }
}

impl Readable for VehicleRegistrationNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VehicleRegistrationNumber> {
        let code_page: CodePage = reader.read_u8()?.into();
        let mut vehicle_reg_number = bytes_to_string(&reader.read_into_vec(VEHICLE_REG_NUMBER_LENGTH)?, &code_page);
        vehicle_reg_number = if code_page == CodePage::Invalid { "".to_owned() } else { vehicle_reg_number };
        Ok(Self { code_page, vehicle_reg_number })
    }
}

impl Serialize for VehicleRegistrationNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.vehicle_reg_number)
    }
}

impl Writable for VehicleRegistrationNumber {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        write_code_page_string::<{ VEHICLE_REG_NUMBER_LENGTH as usize }, _>(
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

    use super::*;
    use crate::Readable;

    #[test]
    fn test_read_vehicle_registration_number() {
        let mut data = vec![0x01]; // CodePage::IsoIec8859_1
        let vrn = "TEST-VRN";
        let mut vrn_bytes = vrn.as_bytes().to_vec();
        vrn_bytes.resize(13, 0x20); // Pad with spaces
        data.extend_from_slice(&vrn_bytes);

        let mut reader = BinMemoryBuffer::from(data);

        let result = VehicleRegistrationNumber::read(&mut reader);

        assert!(result.is_ok());
        let vrn_data = result.unwrap();
        assert_eq!(vrn_data.code_page, CodePage::IsoIec8859_1);
        assert_eq!(vrn_data.vehicle_reg_number, vrn);
    }

    #[test]
    fn test_read_vehicle_registration_number_invalid_code_page() {
        let mut data = vec![0xFF]; // CodePage::Invalid
        let vrn = "GARBAGE-DATA";
        let mut vrn_bytes = vrn.as_bytes().to_vec();
        vrn_bytes.resize(13, 0x20);
        data.extend_from_slice(&vrn_bytes);

        let mut reader = BinMemoryBuffer::from(data);

        let result = VehicleRegistrationNumber::read(&mut reader);

        assert!(result.is_ok());
        let vrn_data = result.unwrap();
        assert_eq!(vrn_data.code_page, CodePage::Invalid);
        assert_eq!(vrn_data.vehicle_reg_number, "");
    }

    #[test]
    fn test_vehicle_registration_number_serialization() {
        let vrn = VehicleRegistrationNumber { code_page: CodePage::IsoIec8859_1, vehicle_reg_number: "TEST-VRN".to_string() };

        let serialized = serde_json::to_string(&vrn).unwrap();
        assert_eq!(serialized, "\"TEST-VRN\"");
    }

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
