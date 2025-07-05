use crate::{
    Readable,
    helpers::vec_u8_to_string,
    tacho::{EmbedderIcAssemblerId, ExtendedSerialNumber},
};

#[derive(Debug)]
pub struct CardIccIdentification {
    pub clock_stop: u8,
    pub card_serial_number: ExtendedSerialNumber,
    pub card_approval_number: String,
    pub card_personaliser_id: u8,
    pub embedder_ic_assembler_id: EmbedderIcAssemblerId,
    pub ic_identifier: Vec<u8>,
}

impl Readable<CardIccIdentification> for CardIccIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardIccIdentification> {
        let clock_stop = reader.read_u8()?;
        let card_serial_number = ExtendedSerialNumber::read(reader)?;
        let card_approval_number = vec_u8_to_string(reader.read_into_vec(8)?)?;
        let card_personaliser_id = reader.read_u8()?;
        let embedder_ic_assembler_id = EmbedderIcAssemblerId::read(reader)?;
        let ic_identifier = reader.read_into_vec(2)?;

        Ok(Self {
            clock_stop,
            card_serial_number,
            card_approval_number,
            card_personaliser_id,
            embedder_ic_assembler_id,
            ic_identifier,
        })
    }
}
