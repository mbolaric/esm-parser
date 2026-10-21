use serde::Serialize;

use crate::{BCDString, Readable, bytes_to_ia5_fix_string};

/// Provides information about the IC embedder.
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct EmbedderIcAssemblerId {
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "moduleEmbedder")]
    pub module_embedder: String,
    #[serde(rename = "manufacturerInformation")]
    pub manufacturer_information: Vec<u8>,
}

impl EmbedderIcAssemblerId {
    pub(crate) const MANUFACTURER_INFORMATION_LENGTH: u32 = 2;
    pub(crate) const COUNTRY_CODE_LENGTH: u32 = 2;
    pub(crate) const MODULE_EMBEDDER_LENGTH: usize = 1;
}

impl Readable for EmbedderIcAssemblerId {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<EmbedderIcAssemblerId> {
        let country_code = bytes_to_ia5_fix_string(&reader.read_into_vec(EmbedderIcAssemblerId::COUNTRY_CODE_LENGTH)?)?;
        let module_embedder = BCDString::decode(&reader.read_bytes::<{ EmbedderIcAssemblerId::MODULE_EMBEDDER_LENGTH }>()?)?;
        // OCTET STRING(SIZE(l))
        // You should interpret it as two raw bytes that represent a manufacturer code (not a printable character). So:
        //  - Do not decode it as ASCII or UTF-8
        //  - Instead, treat it like a numeric or binary ID
        //  - usually interpreted as a hex code
        let manufacturer_information = reader.read_into_vec(EmbedderIcAssemblerId::MANUFACTURER_INFORMATION_LENGTH)?;

        Ok(Self { country_code, module_embedder, manufacturer_information })
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn test_embedder_ic_assembler_id_read() {
        let data = vec![b'D', b'E', 0x12, 0xAA, 0xBB];
        let mut reader = BinMemoryBuffer::from(data.clone());
        let id = EmbedderIcAssemblerId::read(&mut reader).unwrap();
        assert_eq!(id.country_code, "DE");
        assert_eq!(id.module_embedder, "12");
        assert_eq!(id.manufacturer_information, vec![0xAA, 0xBB]);
    }
}
