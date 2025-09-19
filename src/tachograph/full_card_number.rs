use serde::Serialize;

use crate::{
    CodePage, Readable, bytes_to_string,
    tacho::{EquipmentType, NationNumeric},
};

/// Code fully identifying a tachograph card.
#[derive(Debug, Serialize)]
pub struct FullCardNumber {
    #[serde(rename = "cardType")]
    pub card_type: EquipmentType,
    #[serde(rename = "cardIssuingMemberState")]
    pub card_issuing_member_state: NationNumeric,
    #[serde(rename = "cardNumber")]
    pub card_number: String,
}

impl Readable<FullCardNumber> for FullCardNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<FullCardNumber> {
        let card_type: EquipmentType = reader.read_u8()?.into();
        let mut card_issuing_member_state: NationNumeric = reader.read_u8()?.into();
        let mut card_number: String = bytes_to_string(&reader.read_into_vec(16)?, &CodePage::IsoIec8859_1);
        if card_type == EquipmentType::NullCard {
            card_issuing_member_state = NationNumeric::Unknown;
            card_number = "".to_owned();
        }
        Ok(Self { card_type, card_issuing_member_state, card_number })
    }
}
