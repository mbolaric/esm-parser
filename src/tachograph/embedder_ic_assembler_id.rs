use serde::Serialize;

use crate::{BCDString, Readable, bytes_to_ia5_fix_string};

const MANUFACTURER_INFORMATION_LENGTH: u32 = 2;
const COUNTRY_CODE_LENGTH: u32 = 2;
const MODULE_EMBEDDER_LENGTH: usize = 1;

/// Provides information about the IC embedder.
#[derive(Debug, Serialize)]
pub struct EmbedderIcAssemblerId {
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "moduleEmbedder")]
    pub module_embedder: String,
    #[serde(rename = "manufacturerInformation")]
    pub manufacturer_information: Vec<u8>,
}

impl Readable<EmbedderIcAssemblerId> for EmbedderIcAssemblerId {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<EmbedderIcAssemblerId> {
        let country_code = bytes_to_ia5_fix_string(&reader.read_into_vec(COUNTRY_CODE_LENGTH)?)?;
        let module_embedder = BCDString::decode(&reader.read_bytes::<MODULE_EMBEDDER_LENGTH>()?)?;
        // OCTET STRING(SIZE(l))
        // You should interpret it as two raw bytes that represent a manufacturer code (not a printable character). So:
        //  - Do not decode it as ASCII or UTF-8
        //  - Instead, treat it like a numeric or binary ID
        //  - usually interpreted as a hex code
        let manufacturer_information = reader.read_into_vec(MANUFACTURER_INFORMATION_LENGTH)?;

        Ok(Self { country_code, module_embedder, manufacturer_information })
    }
}
