use serde::Serialize;

use crate::{CodePage, Readable, bytes_to_string};

/// Registration number of the vehicle (VRN). The registration number is
/// assigned by the vehicle licensing authority.
#[derive(Debug, PartialEq)]
pub struct VehicleRegistrationNumber {
    pub code_page: CodePage,
    pub vehicle_reg_number: String,
}

impl Readable<VehicleRegistrationNumber> for VehicleRegistrationNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VehicleRegistrationNumber> {
        let code_page: CodePage = reader.read_u8()?.into();
        let mut vehicle_reg_number = bytes_to_string(&reader.read_into_vec(13)?, &code_page);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Readable;
    use binary_data::BinMemoryBuffer;

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
}
