use serde::Serialize;

use crate::tacho::{ControlType, FullCardNumber, TimeReal, VehicleRegistrationIdentification};
use crate::{Readable, Validatable, Writable};

/// Information, stored in a driver or workshop card, related to the last
/// control the driver has been subject to (Annex 1C requirements 274,
/// 299, 327, and 350).
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardControlActivityDataRecord {
    #[serde(rename = "controlType")]
    pub control_type: ControlType,
    #[serde(rename = "controlTime")]
    pub control_time: TimeReal,
    /// The FullCardNumber of the control officer having performed the control.
    #[serde(rename = "controlCardNumber")]
    pub control_card_number: FullCardNumber,
    /// The VRN and registering Member State of the vehicle in which the control happened.
    #[serde(rename = "controlVehicleRegistration")]
    pub control_vehicle_registration: VehicleRegistrationIdentification,
    #[serde(rename = "controlDownloadPeriodBegin")]
    pub control_download_period_begin: TimeReal,
    #[serde(rename = "controlDownloadPeriodEnd")]
    pub control_download_period_end: TimeReal,
}

impl Validatable for CardControlActivityDataRecord {
    fn validate(&self) -> crate::Result<()> {
        self.control_time.validate()?;
        self.control_card_number.validate()?;
        self.control_vehicle_registration.validate()?;
        self.control_download_period_begin.validate()?;
        self.control_download_period_end.validate()?;
        Ok(())
    }
}

impl Readable for CardControlActivityDataRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardControlActivityDataRecord> {
        let control_type: ControlType = reader.read_u8()?.into();
        let control_time = TimeReal::read(reader)?;
        let control_card_number = FullCardNumber::read(reader)?;
        let control_vehicle_registration = VehicleRegistrationIdentification::read(reader)?;
        let control_download_period_begin = TimeReal::read(reader)?;
        let control_download_period_end = TimeReal::read(reader)?;
        Ok(Self {
            control_type,
            control_time,
            control_card_number,
            control_vehicle_registration,
            control_download_period_begin,
            control_download_period_end,
        })
    }
}

impl Writable for CardControlActivityDataRecord {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.control_type as u8)?;
        self.control_time.write_raw(writer)?;
        self.control_card_number.write_raw(writer)?;
        self.control_vehicle_registration.write_raw(writer)?;
        self.control_download_period_begin.write_raw(writer)?;
        self.control_download_period_end.write_raw(writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;
    use crate::CodePage;
    use crate::tacho::{EquipmentType, NationNumeric, VehicleRegistrationNumber};

    #[test]
    fn test_card_control_activity_data_record_roundtrip() {
        let mut data = vec![0x00]; // control_type
        data.extend_from_slice(&[0x00, 0x00, 0x01, 0x00]); // control_time
        // FullCardNumber: 1 byte type, 1 byte nation, 16 bytes number
        data.extend_from_slice(&[0x03, 0x01]);
        data.extend_from_slice(b"CTRL123456789012");
        // VehicleRegistrationIdentification: 1 byte nation, 1 byte codepage, 13 bytes VRN
        data.extend_from_slice(&[0x01, 0x01]);
        let mut vrn = b"TEST-VRN".to_vec();
        vrn.resize(13, 0x20);
        data.extend_from_slice(&vrn);
        // controlDownloadPeriodBegin & End: 4 bytes each
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x10]);
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x20]);

        let mut reader = BinMemoryBuffer::from(data.clone());
        let rec = CardControlActivityDataRecord::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        rec.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_card_control_activity_data_record_validation_and_zero_byte_emission() {
        let invalid = CardControlActivityDataRecord {
            control_type: ControlType::CardDownloaded,
            control_time: TimeReal::new(100),
            control_card_number: FullCardNumber {
                card_type: EquipmentType::ControlCard,
                card_issuing_member_state: NationNumeric::Austria,
                card_number: "CTRL123".into(),
            },
            control_vehicle_registration: VehicleRegistrationIdentification {
                vehicle_registration_nation: NationNumeric::Austria,
                vehicle_registration_number: VehicleRegistrationNumber {
                    code_page: CodePage::Invalid,
                    vehicle_reg_number: "BAD".into(),
                },
            },
            control_download_period_begin: TimeReal::new(10),
            control_download_period_end: TimeReal::new(20),
        };
        assert!(invalid.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(invalid.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
