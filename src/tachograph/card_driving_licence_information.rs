use crate::{
    Error, Readable,
    helpers::vec_u8_to_string,
    tacho::{Name, NationNumeric},
};

#[derive(Debug)]
pub struct CardDrivingLicenceInformation {
    pub driving_licence_issuing_authority: Name,
    pub driving_licence_issuing_nation: NationNumeric,
    pub driving_licence_number: String,
}

impl Readable<CardDrivingLicenceInformation> for CardDrivingLicenceInformation {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardDrivingLicenceInformation> {
        let driving_licence_issuing_authority = Name::read(reader)?;
        let driving_licence_issuing_nation: NationNumeric = reader.read_u8()?.into();
        let driving_licence_number = vec_u8_to_string(reader.read_into_vec(16)?)?;
        if driving_licence_issuing_authority.name.len() != 0 && driving_licence_number.is_empty() {
            return Err(Error::CorruptedDrivingLicenceNumber);
        }

        Ok(Self { driving_licence_issuing_authority, driving_licence_issuing_nation, driving_licence_number })
    }
}
