use crate::tacho::CardSlotStatus;
use crate::writer::{Validatable, Writable};

impl Validatable for CardSlotStatus {
    fn validate(&self) -> crate::Result<()> {
        if self.data != 0 {
            let low = self.data & 0xF;
            let high = (self.data >> 4) & 0xF;
            if low > 4 || high > 4 {
                return Err(crate::Error::ValidationError(format!(
                    "CardSlotStatus raw data (0x{:02X}) contains undefined slot status nibble",
                    self.data
                )));
            }
            if low != self.driver_slot as u8 || high != self.co_driver_slot as u8 {
                return Err(crate::Error::ValidationError(format!(
                    "CardSlotStatus raw data (0x{:02X}) is inconsistent with driver_slot ({:?}) / co_driver_slot ({:?})",
                    self.data, self.driver_slot, self.co_driver_slot
                )));
            }
        }
        Ok(())
    }
}

impl Writable for CardSlotStatus {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        let val = if self.data != 0 || (self.driver_slot as u8 == 0 && self.co_driver_slot as u8 == 0) {
            self.data
        } else {
            ((self.co_driver_slot as u8) << 4) | (self.driver_slot as u8)
        };
        writer.write_u8(val)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::Readable;
    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};

    #[test]
    fn test_card_slot_status_roundtrip() {
        let data = vec![0x21]; // co_driver 2, driver 1
        let mut reader = BinMemoryBuffer::from(data.clone());
        let slot = CardSlotStatus::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        slot.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_card_slot_status_validation_and_zero_byte_emission() {
        let invalid = CardSlotStatus {
            data: 0xF1,
            driver_slot: CardSlotStatusType::DriverCard,
            co_driver_slot: CardSlotStatusType::Unknown,
        };
        assert!(invalid.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(invalid.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
