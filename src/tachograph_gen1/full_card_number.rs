use crate::{
    helpers::vec_u8_to_string,
    tacho::{EquipmentTypeCode, NationNumericCode},
    Readable,
};

#[derive(Debug)]
pub struct FullCardNumber {
    pub card_type: EquipmentTypeCode,
    pub card_issuing_member_state: NationNumericCode,
    pub card_number: String,
}

impl Readable<FullCardNumber> for FullCardNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<FullCardNumber> {
        let card_type: EquipmentTypeCode = reader.read_u8()?.into();
        let mut card_issuing_member_state: NationNumericCode = reader.read_u8()?.into();
        let mut card_number: String = vec_u8_to_string(reader.read_into_vec(16)?)?;
        if card_type == EquipmentTypeCode::NullCard {
            card_issuing_member_state = NationNumericCode::Unknown;
            card_number = "".to_owned();
        }
        Ok(Self {
            card_type,
            card_issuing_member_state,
            card_number,
        })
    }
}
