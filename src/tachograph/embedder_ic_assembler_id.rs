use crate::{BCDString, Readable, bytes_to_ia5_fix_string};

#[derive(Debug)]
pub struct EmbedderIcAssemblerId {
    pub country_code: String,
    pub module_embedder: String,
    pub manufacturer_information: Vec<u8>,
}

impl Readable<EmbedderIcAssemblerId> for EmbedderIcAssemblerId {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<EmbedderIcAssemblerId> {
        let country_code = bytes_to_ia5_fix_string(&reader.read_into_vec(2)?)?;
        let module_embedder = BCDString::decode(&reader.read_bytes::<1>()?)?;
        // OCTET STRING(SIZE(l))
        // You should interpret it as two raw bytes that represent a manufacturer code (not a printable character). So:
        //  - Do not decode it as ASCII or UTF-8
        //  - Instead, treat it like a numeric or binary ID
        //  - usually interpreted as a hex code
        let manufacturer_information = reader.read_into_vec(2)?;

        Ok(Self { country_code, module_embedder, manufacturer_information })
    }
}
