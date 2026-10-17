use serde::Serialize;

use crate::tacho::{NationNumeric, VehicleRegistrationNumber};
use crate::{Readable, Validatable, Writable};

/// Identification of a vehicle, unique for Europe (VRN and Member State).
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VehicleRegistrationIdentification {
    #[serde(rename = "vehicleRegistrationNation")]
    pub vehicle_registration_nation: NationNumeric,
    #[serde(rename = "vehicleRegistrationNumber")]
    pub vehicle_registration_number: VehicleRegistrationNumber,
}

impl Validatable for VehicleRegistrationIdentification {
    fn validate(&self) -> crate::Result<()> {
        self.vehicle_registration_number.validate()
    }
}

impl Readable for VehicleRegistrationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<VehicleRegistrationIdentification> {
        let nation_numeric: NationNumeric = reader.read_u8()?.into();
        let vehicle_registration_number = VehicleRegistrationNumber::read(reader)?;

        Ok(Self { vehicle_registration_nation: nation_numeric, vehicle_registration_number })
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

    use super::*;
    use crate::CodePage;

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
