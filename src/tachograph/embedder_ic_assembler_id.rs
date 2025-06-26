use crate::{Readable, gen1::BCDString, helpers::vec_u8_to_string};

#[derive(Debug)]
pub struct EmbedderIcAssemblerId {
    pub country_code: String,
    pub module_embedder: String,
    pub manufacturer_information: Vec<u8>,
}

impl Readable<EmbedderIcAssemblerId> for EmbedderIcAssemblerId {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<EmbedderIcAssemblerId> {
        let country_code = vec_u8_to_string(reader.read_into_vec(2)?)?;
        let module_embedder = BCDString::decode(&reader.read_bytes::<1>()?);
        let manufacturer_information = reader.read_into_vec(2)?;

        Ok(Self { country_code, module_embedder, manufacturer_information })
    }
}
