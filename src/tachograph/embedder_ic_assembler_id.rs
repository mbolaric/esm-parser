use serde::Serialize;

use crate::{
    BCDString, Readable, Validatable, Writable, bytes_to_ia5_fix_string, validate_bcd, validate_fixed_bytes,
    validate_ia5_string, write_ia5_string,
};

const MANUFACTURER_INFORMATION_LENGTH: u32 = 2;
const COUNTRY_CODE_LENGTH: u32 = 2;
const MODULE_EMBEDDER_LENGTH: usize = 1;

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

impl Validatable for EmbedderIcAssemblerId {
    fn validate(&self) -> crate::Result<()> {
        validate_ia5_string::<{ COUNTRY_CODE_LENGTH as usize }>("EmbedderIcAssemblerId.country_code", &self.country_code)?;
        validate_bcd::<MODULE_EMBEDDER_LENGTH>("EmbedderIcAssemblerId.module_embedder", &self.module_embedder)?;
        validate_fixed_bytes::<{ MANUFACTURER_INFORMATION_LENGTH as usize }>(
            "EmbedderIcAssemblerId.manufacturer_information",
            &self.manufacturer_information,
        )
    }
}

impl Readable for EmbedderIcAssemblerId {
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

impl Writable for EmbedderIcAssemblerId {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        write_ia5_string::<{ COUNTRY_CODE_LENGTH as usize }, _>(
            writer,
            "EmbedderIcAssemblerId.country_code",
            &self.country_code,
        )?;
        let embedder_bytes = validate_bcd::<MODULE_EMBEDDER_LENGTH>(
            "EmbedderIcAssemblerId.module_embedder",
            &self.module_embedder,
        )?;
        writer.write_all(&embedder_bytes)?;
        writer.write_all(&self.manufacturer_information)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn test_embedder_ic_assembler_id_roundtrip() {
        let data = vec![b'D', b'E', 0x12, 0xAA, 0xBB];
        let mut reader = BinMemoryBuffer::from(data.clone());
        let id = EmbedderIcAssemblerId::read(&mut reader).unwrap();
        assert_eq!(id.country_code, "DE");
        assert_eq!(id.module_embedder, "12");
        assert_eq!(id.manufacturer_information, vec![0xAA, 0xBB]);

        let mut writer = BinMemoryBuffer::new();
        id.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_embedder_ic_assembler_id_validation_and_zero_byte_emission() {
        // Invalid BCD in module_embedder after valid country_code
        let invalid_bcd = EmbedderIcAssemblerId {
            country_code: "DE".into(),
            module_embedder: "AB".into(),
            manufacturer_information: vec![0x01, 0x02],
        };
        assert!(invalid_bcd.validate().is_err());
        let mut writer = BinMemoryBuffer::new();
        assert!(invalid_bcd.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);

        // Wrong manufacturer_information length
        let invalid_man = EmbedderIcAssemblerId {
            country_code: "DE".into(),
            module_embedder: "12".into(),
            manufacturer_information: vec![0x01],
        };
        assert!(invalid_man.validate().is_err());
        let mut writer2 = BinMemoryBuffer::new();
        assert!(invalid_man.write(&mut writer2).is_err());
        assert_eq!(writer2.as_slice().len(), 0);
    }
}
