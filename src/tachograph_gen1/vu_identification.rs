use crate::{
    gen1::{Address, Name, SerialNumber, SoftwareIdentification},
    helpers::vec_u8_to_string,
    tacho::TimeReal,
    Readable,
};

#[derive(Debug)]
pub struct VUIdentification {
    pub manufacturer_name: Name,
    pub manufacturer_address: Address,
    pub part_number: String,
    pub serial_number: SerialNumber,
    pub software_identification: SoftwareIdentification,
    pub manufacturing_date: TimeReal,
    pub approval_number: String,
}

impl Readable<VUIdentification> for VUIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<VUIdentification> {
        let manufacturer_name = Name::read(reader)?;
        let manufacturer_address = Address::read(reader)?;
        let part_number = vec_u8_to_string(reader.read_into_vec(16)?)?
            .trim()
            .to_string();
        let serial_number = SerialNumber::read(reader)?;
        let software_identification = SoftwareIdentification::read(reader)?;

        let manufacturing_date = TimeReal::read(reader)?;
        let approval_number = vec_u8_to_string(reader.read_into_vec(8)?)?
            .trim()
            .to_string();

        Ok(Self {
            manufacturer_name,
            manufacturer_address,
            part_number,
            serial_number,
            software_identification,
            manufacturing_date,
            approval_number,
        })
    }
}
