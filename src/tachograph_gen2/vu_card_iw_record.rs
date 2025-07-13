use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::{FullCardNumberAndGeneration, PreviousVehicleInfo},
    tacho::{CardSlotNumber, HolderName, ManualInputFlag, OdometerShort, TimeReal},
};

#[derive(Debug)]
pub struct VuCardIWRecord {
    pub card_holder_name: HolderName,
    pub full_card_number_and_generation: FullCardNumberAndGeneration,
    pub card_expiry_date: TimeReal,
    pub card_insertion_time: TimeReal,
    pub vehicle_odometer_value_at_insertion: OdometerShort,
    pub card_slot_number: CardSlotNumber,
    pub card_withdrawal_time: TimeReal,
    pub vehicle_odometer_value_at_withdrawal: OdometerShort,
    pub previous_wehicle_info: PreviousVehicleInfo,
    pub manual_input_flag: ManualInputFlag,
}

impl Readable<VuCardIWRecord> for VuCardIWRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuCardIWRecord> {
        let card_holder_name = HolderName::read(reader)?;
        let full_card_number_and_generation = FullCardNumberAndGeneration::read(reader)?;
        let card_expiry_date = TimeReal::read(reader)?;
        let card_insertion_time = TimeReal::read(reader)?;
        let vehicle_odometer_value_at_insertion = OdometerShort::read(reader)?;
        let card_slot_number: CardSlotNumber = reader.read_u8()?.into();
        let card_withdrawal_time = TimeReal::read(reader)?;
        let vehicle_odometer_value_at_withdrawal = OdometerShort::read(reader)?;
        let previous_wehicle_info = PreviousVehicleInfo::read(reader)?;
        let manual_input_flag: ManualInputFlag = reader.read_u8()?.into();

        Ok(Self {
            card_holder_name,
            full_card_number_and_generation,
            card_expiry_date,
            card_insertion_time,
            vehicle_odometer_value_at_insertion,
            card_slot_number,
            card_withdrawal_time,
            vehicle_odometer_value_at_withdrawal,
            previous_wehicle_info,
            manual_input_flag,
        })
    }
}
