use serde::Serialize;

use crate::{
    CodePage, Readable, bytes_to_string,
    tacho::{EmbedderIcAssemblerId, ExtendedSerialNumber},
};

/// Information, stored in a card, related to the identification of the integrated
/// circuit (IC) card (Annex 1C requirement 248).
#[derive(Debug, Serialize)]
pub struct CardIccIdentification {
    #[serde(rename = "clockStop")]
    pub clock_stop: u8,
    #[serde(rename = "cardExtendedSerialNumber")]
    pub card_extended_serial_number: ExtendedSerialNumber,
    #[serde(rename = "cardApprovalNumber")]
    pub card_approval_number: String,
    #[serde(rename = "cardPersonaliserID")]
    pub card_personaliser_id: u8,
    #[serde(rename = "embedderIcAssemblerId")]
    pub embedder_ic_assembler_id: EmbedderIcAssemblerId,
    #[serde(rename = "icIdentifier")]
    pub ic_identifier: Vec<u8>,
}

impl Readable<CardIccIdentification> for CardIccIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardIccIdentification> {
        let clock_stop = reader.read_u8()?;
        let card_extended_serial_number = ExtendedSerialNumber::read(reader)?;
        // It's a string encoded using IA5, which corresponds to ASCII characters.
        let card_approval_number = bytes_to_string(&reader.read_into_vec(8)?, &CodePage::IsoIec8859_1);
        let card_personaliser_id = reader.read_u8()?;
        let embedder_ic_assembler_id = EmbedderIcAssemblerId::read(reader)?;
        // OCTET STRING(SIZE(l))
        // You should interpret it as two raw bytes that represent a Identifier of the IC on the card (not a printable character). So:
        //  - Do not decode it as ASCII or UTF-8
        //  - Instead, treat it like a numeric or binary ID
        //  - usually interpreted as a hex code
        let ic_identifier = reader.read_into_vec(2)?;

        Ok(Self {
            clock_stop,
            card_extended_serial_number,
            card_approval_number,
            card_personaliser_id,
            embedder_ic_assembler_id,
            ic_identifier,
        })
    }
}
