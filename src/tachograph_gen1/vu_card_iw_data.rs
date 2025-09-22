use binary_data::BigEndian;
use serde::Serialize;

use crate::{
    Readable,
    gen1::PreviousVehicleInfo,
    tacho::{CardSlotNumber, FullCardNumber, HolderName, ManualInputFlag, OdometerShort, TimeReal},
};

/// Information, stored in a vehicle unit, related to an insertion and withdrawal
/// cycle of a driver card or of a workshop card in the vehicle unit
/// (Annex 1B requirement 081 and Annex 1C requirement 102).
#[derive(Debug, Serialize)]
pub struct VuCardIWRecord {
    #[serde(rename = "cardHolderName")]
    pub card_holder_name: HolderName,
    #[serde(rename = "fullCardNumber")]
    pub full_card_number: FullCardNumber,
    #[serde(rename = "cardExpiryDate")]
    pub card_expiry_date: TimeReal,
    #[serde(rename = "cardInsertionTime")]
    pub card_insertion_time: TimeReal,
    #[serde(rename = "vehicleOdometerValueAtInsertion")]
    pub vehicle_odometer_value_at_insertion: OdometerShort,
    #[serde(rename = "cardSlotNumber")]
    pub card_slot_number: CardSlotNumber,
    #[serde(rename = "cardWithdrawalTime")]
    pub card_withdrawal_time: TimeReal,
    #[serde(rename = "vehicleOdometerValueAtWithdrawal")]
    pub vehicle_odometer_value_at_withdrawal: OdometerShort,
    #[serde(rename = "previousVehicleInfo")]
    pub previous_vehicle_info: PreviousVehicleInfo,
    #[serde(rename = "manualInputFlag")]
    pub manual_input_flag: ManualInputFlag,
}

impl Readable<VuCardIWRecord> for VuCardIWRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuCardIWRecord> {
        let card_holder_name = HolderName::read(reader)?;
        let full_card_number = FullCardNumber::read(reader)?;
        let card_expiry_date = TimeReal::read(reader)?;
        let card_insertion_time = TimeReal::read(reader)?;
        let vehicle_odometer_value_at_insertion = OdometerShort::read(reader)?;
        let card_slot_number: CardSlotNumber = reader.read_u8()?.into();
        let card_withdrawal_time = TimeReal::read(reader)?;
        let vehicle_odometer_value_at_withdrawal = OdometerShort::read(reader)?;
        let previous_vehicle_info = PreviousVehicleInfo::read(reader)?;
        let manual_input_flag: ManualInputFlag = reader.read_u8()?.into();
        Ok(Self {
            card_holder_name,
            full_card_number,
            card_expiry_date,
            card_insertion_time,
            vehicle_odometer_value_at_insertion,
            card_slot_number,
            card_withdrawal_time,
            vehicle_odometer_value_at_withdrawal,
            previous_vehicle_info,
            manual_input_flag,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct VuCardIWData {
    pub no_of_iw_records: u16,
    pub vu_card_iw_records: Vec<VuCardIWRecord>,
}

impl Readable<VuCardIWData> for VuCardIWData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuCardIWData> {
        let no_of_iw_records = reader.read_u16::<BigEndian>()?;
        let mut vu_card_iw_records: Vec<VuCardIWRecord> = Vec::new();
        for _ in 0..no_of_iw_records {
            let record = VuCardIWRecord::read(reader)?;
            vu_card_iw_records.push(record);
        }
        Ok(Self { no_of_iw_records, vu_card_iw_records })
    }
}
