use crate::Readable;

#[derive(Debug)]
pub struct CardChipIdentification {
    pub ic_serial_number: Vec<u8>,
    pub ic_manufacturing_references: Vec<u8>,
}

impl Readable<CardChipIdentification> for CardChipIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<CardChipIdentification> {
        let ic_serial_number = reader.read_into_vec(4)?;
        let ic_manufacturing_references = reader.read_into_vec(4)?;

        Ok(Self { ic_serial_number, ic_manufacturing_references })
    }
}
