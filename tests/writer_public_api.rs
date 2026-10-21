#![cfg(feature = "write")]

use binary_data::{BinMemoryBuffer, BinSeek, WriteBytes};
use esm_parser::tacho::{CardSlotStatus, Name, OdometerShort, TimeReal, VehicleRegistrationNumber};
use esm_parser::writer::{
    SignaturePolicy, SignatureSigner, Validatable, Writable, WritableWithParams, WriteOptions, encode_code_page_padded,
    encode_ia5_padded, ia5_fix_string_to_bytes, string_to_bytes, try_encode_byte, try_string_to_bytes, validate_bcd,
    validate_code_page_string, validate_fixed_bytes, validate_ia5_string, write_code_page_string, write_ia5_string,
    write_records,
};
use esm_parser::{BCDString, CodePage, Readable, Result};

#[test]
fn public_writers_cover_all_three_groups() {
    assert_eq!(TimeReal::new(256).to_bytes().unwrap(), [0, 0, 1, 0]);
    let mut reader = BinMemoryBuffer::from(vec![0x21]);
    assert_eq!(CardSlotStatus::read(&mut reader).unwrap().to_bytes().unwrap(), [0x21]);
    let vehicle = VehicleRegistrationNumber { code_page: CodePage::IsoIec8859_1, vehicle_reg_number: "AB-123".into() };
    assert_eq!(vehicle.to_bytes().unwrap(), b"\x01AB-123       ");
}

#[test]
fn public_codecs_and_helpers_keep_their_bytes() {
    let cp = CodePage::IsoIec8859_1;
    assert_eq!(string_to_bytes("ü", &cp), [0xFC]);
    assert_eq!(try_string_to_bytes("ü", &cp).unwrap(), [0xFC]);
    assert_eq!(try_encode_byte('ü', &cp).unwrap(), 0xFC);
    assert_eq!(ia5_fix_string_to_bytes("AB").unwrap(), b"AB");
    assert_eq!(encode_code_page_padded::<3>("name", "ü", &cp).unwrap(), [0xFC, b' ', b' ']);
    assert_eq!(encode_ia5_padded::<3>("name", "AB").unwrap(), *b"AB ");
    validate_code_page_string::<3>("name", "ü", &cp).unwrap();
    validate_ia5_string::<3>("name", "AB").unwrap();
    validate_fixed_bytes::<2>("bytes", b"AB").unwrap();
    assert_eq!(validate_bcd::<2>("date", "2024").unwrap(), [0x20, 0x24]);
    assert_eq!(BCDString::encode("123").unwrap(), [0x01, 0x23]);
    assert_eq!(BCDString::encode_strict::<2>("1234").unwrap(), [0x12, 0x34]);

    let mut buffer = BinMemoryBuffer::new();
    write_code_page_string::<3, _>(&mut buffer, "name", &cp, "ü").unwrap();
    write_ia5_string::<3, _>(&mut buffer, "name", "AB").unwrap();
    write_records(&mut buffer, &[TimeReal::new(256)], |w, record| record.write(w)).unwrap();
    assert_eq!(buffer.as_slice(), &[1, 0xFC, b' ', b' ', b'A', b'B', b' ', 0, 0, 1, 0]);
}

#[test]
fn slices_and_vectors_validate_before_touching_existing_output() {
    let mut buffer = BinMemoryBuffer::from(vec![0xAA]);
    buffer.seek(1).unwrap();
    let invalid = vec![OdometerShort { data: Some(1) }, OdometerShort { data: Some(10_000_000) }];
    assert!(invalid.write(&mut buffer).is_err());
    assert!(invalid.as_slice().write(&mut buffer).is_err());
    assert_eq!(buffer.as_slice(), &[0xAA]);

    let valid = vec![TimeReal::new(1), TimeReal::new(2)];
    assert_eq!(valid.to_bytes().unwrap(), valid.as_slice().to_bytes().unwrap());
    assert!(None::<Name>.validate().is_ok());
}

struct WithOptions;

impl Validatable for WithOptions {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Writable for WithOptions {
    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(0)?;
        Ok(())
    }

    fn write_raw_with_options<W: WriteBytes + BinSeek>(&self, writer: &mut W, options: &WriteOptions) -> Result<()> {
        writer.write_u8(u8::from(matches!(options.signature_policy, SignaturePolicy::Strip)))?;
        Ok(())
    }
}

impl WritableWithParams for WithOptions {
    type P = u8;

    fn write_raw_with_options<W: WriteBytes + BinSeek>(
        &self,
        writer: &mut W,
        params: &Self::P,
        _options: &WriteOptions,
    ) -> Result<()> {
        writer.write_u8(*params)?;
        Ok(())
    }
}

#[test]
fn public_options_and_parameterized_contract_remain_usable() {
    assert!(matches!(WriteOptions::default().signature_policy, SignaturePolicy::Keep));
    assert!(matches!(WriteOptions::keep_signatures().signature_policy, SignaturePolicy::Keep));
    let options = WriteOptions::strip_signatures();
    assert_eq!(vec![WithOptions, WithOptions].to_bytes_with_options(&options).unwrap(), [1, 1]);
    assert_eq!(WithOptions.to_bytes().unwrap(), [0]);
    assert!(
        matches!(WriteOptions::with_custom_signature(vec![7]).signature_policy, SignaturePolicy::Custom(bytes) if bytes == [7])
    );
    let SignaturePolicy::Sign(signer): SignaturePolicy = WriteOptions::with_signer(|bytes| Ok(bytes.to_vec())).signature_policy
    else {
        panic!("expected a signer");
    };
    let signer: SignatureSigner = signer;
    assert_eq!(signer(&[7]).unwrap(), [7]);
    let mut buffer = BinMemoryBuffer::new();
    WritableWithParams::write(&WithOptions, &mut buffer, &42).unwrap();
    assert_eq!(buffer.as_slice(), &[42]);
}
