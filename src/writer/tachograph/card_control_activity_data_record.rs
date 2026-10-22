use crate::tacho::CardControlActivityDataRecord;
use crate::writer::{Validatable, Writable};

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

impl Writable for CardControlActivityDataRecord {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.control_type.bits())?;
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

    use crate::tacho::{EquipmentType, NationNumeric, VehicleRegistrationNumber, *};
    use crate::writer::{Validatable, Writable};
    use crate::{CodePage, Readable};

    #[test]
    fn test_card_control_activity_data_record_roundtrip() {
        let mut data = vec![0xC0]; // Card and VU downloaded
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
            control_type: ControlType::CARD_DOWNLOADED,
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
