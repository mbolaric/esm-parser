#[path = "common/card.rs"]
mod card;

use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use card::section;
use esm_parser::gen2::VuCardRecord;
use esm_parser::tacho::{CardIdentification, CardNumber, CardNumberParams, EquipmentType, TimeReal};
use esm_parser::{Export, Readable, ReadableWithParams, Result, SerializedTachographData, parse_from_memory};
use serde_json::json;

const CARD_TYPES: [EquipmentType; 4] =
    [EquipmentType::DriverCard, EquipmentType::WorkshopCard, EquipmentType::ControlCard, EquipmentType::CompanyCard];

fn read_number(bytes: &[u8], kind: EquipmentType) -> Result<CardNumber> {
    CardNumber::read(&mut BinMemoryBuffer::from(bytes), &CardNumberParams::new(kind))
}

fn identification(number: &[u8; 16]) -> Vec<u8> {
    let mut bytes = vec![13];
    bytes.extend(number);
    bytes.push(1);
    bytes.extend([b' '; 35]);
    for time in [1_700_000_000_u32, 1_700_000_100, 1_800_000_000] {
        bytes.extend(time.to_be_bytes());
    }
    assert_eq!(bytes.len(), 65);
    bytes
}

fn card(gen1: bool, gen2: bool, number: &[u8; 16]) -> Vec<u8> {
    // Partial driver downloads isolate shared card-number parsing and raw retention.
    let mut bytes = Vec::new();
    section(&mut bytes, 0x0002, 0, &[0; 25]);
    section(&mut bytes, 0x0005, 0, &[0; 8]);
    let mut id = identification(number);
    id.resize(143, 0);
    for (enabled, appendix, signature_size) in [(gen1, 0, 128), (gen2, 2, 64)] {
        if !enabled {
            continue;
        }
        let mut app = if appendix == 0 { vec![1, 0, 0, 6, 12] } else { vec![1, 1, 0, 12, 24] };
        app.extend(13776_u16.to_be_bytes());
        app.extend(200_u16.to_be_bytes());
        if appendix == 0 {
            app.push(112);
        } else {
            for capacity in [112_u16, 336, 112, 200] {
                app.extend(capacity.to_be_bytes());
            }
        }
        section(&mut bytes, 0x0501, appendix, &app);
        section(&mut bytes, 0x0520, appendix, &id);
        section(&mut bytes, 0x0520, appendix + 1, &vec![0x55; signature_size]);
    }
    bytes
}

#[test]
fn thirteen_character_padded_driver_number_returns_data_without_panicking() {
    let result = std::panic::catch_unwind(|| read_number(b"ABCDEFGHIJKLM   ", EquipmentType::DriverCard));
    assert!(result.is_ok(), "Padded card numbers must not panic");
    let number = result.unwrap().unwrap();
    assert_eq!(number.number, "ABCDEFGHIJKLM");
    assert_eq!(number.identification, "ABCDEFGHIJKLM");
    assert_eq!(number.card_replacement_index, "");
    assert_eq!(number.card_renewal_index, "");
}

#[test]
fn all_padded_lengths_preserve_component_positions_and_consume_exactly_16_bytes() {
    for kind in CARD_TYPES {
        for padding in [b' ', 0] {
            for length in 0..=16 {
                let mut bytes = vec![0xCC; 3];
                bytes.extend([b'A'; 16]);
                bytes[3 + length..].fill(padding);
                bytes.extend([0xAA, 0xBB]);
                let mut reader = BinMemoryBuffer::from(bytes);
                reader.seek(3).unwrap();
                let number = CardNumber::read(&mut reader, &CardNumberParams::new(kind)).unwrap();
                assert_eq!(number.number, "A".repeat(length));
                let id_length = if kind == EquipmentType::DriverCard { 14 } else { 13 };
                assert_eq!(number.identification, "A".repeat(length.min(id_length)));
                assert_eq!(
                    number.card_consecutive_index,
                    if kind != EquipmentType::DriverCard && length > 13 { "A" } else { "" }
                );
                assert_eq!(number.card_replacement_index, if length > 14 { "A" } else { "" });
                assert_eq!(number.card_renewal_index, if length > 15 { "A" } else { "" });
                assert_eq!(reader.pos().unwrap(), 19);
                assert_eq!(reader.read_bytes::<2>().unwrap(), [0xAA, 0xBB]);
            }
        }
    }
}

#[test]
fn padding_and_embedded_nuls_do_not_move_index_characters() {
    for (bytes, driver_id, owner_id, normalized) in [
        (b" BCDEFGHIJKLM123", "BCDEFGHIJKLM1", "BCDEFGHIJKLM", "BCDEFGHIJKLM123"),
        (b"AB\0DEFGHIJKLM123", "ABDEFGHIJKLM1", "ABDEFGHIJKLM", "ABDEFGHIJKLM123"),
    ] {
        for kind in CARD_TYPES {
            let number = read_number(bytes, kind).unwrap();
            assert_eq!(number.number, normalized);
            assert_eq!(number.identification, if kind == EquipmentType::DriverCard { driver_id } else { owner_id });
            assert_eq!(number.card_consecutive_index, if kind == EquipmentType::DriverCard { "" } else { "1" });
            assert_eq!(number.card_replacement_index, "2");
            assert_eq!(number.card_renewal_index, "3");
        }
    }
    let number = read_number(b"ABCDEFGHIJKL  12", EquipmentType::DriverCard).unwrap();
    assert_eq!(number.number, "ABCDEFGHIJKL  12");
    assert_eq!(number.identification, "ABCDEFGHIJKL");
    assert_eq!(number.card_replacement_index, "1");
    assert_eq!(number.card_renewal_index, "2");
}

#[test]
fn truncated_and_non_ascii_card_numbers_return_errors() {
    for kind in CARD_TYPES {
        for length in 0..16 {
            assert!(read_number(&b"ABCDEFGHIJKLM123"[..length], kind).is_err());
        }
        for index in 0..16 {
            for invalid in [0x80, 0xC3, 0xFF] {
                let mut bytes = *b"ABCDEFGHIJKLM123";
                bytes[index] = invalid;
                assert!(read_number(&bytes, kind).is_err(), "{kind:?}, byte {index}");
            }
        }
    }
}

#[test]
fn identification_and_vu_record_callers_preserve_following_fields_with_padded_numbers() {
    for kind in CARD_TYPES {
        let mut bytes = identification(b"ABCDEFGHIJKLM   ");
        bytes.push(0xAA);
        let mut reader = BinMemoryBuffer::from(bytes);
        let id = CardIdentification::read(&mut reader, &CardNumberParams::new(kind)).unwrap();
        assert_eq!(id.card_number.identification, "ABCDEFGHIJKLM");
        assert_eq!(id.card_issue_date.data, 1_700_000_000);
        assert_eq!(id.card_validity_begin.data, 1_700_000_100);
        assert_eq!(id.card_expiry_date.data, 1_800_000_000);
        assert_eq!(reader.read_u8().unwrap(), 0xAA);
    }
    let mut bytes = vec![1, 13];
    bytes.extend(b"ABCDEFGHIJKLM123");
    bytes.push(2);
    bytes.extend([0; 8]);
    bytes.extend([1, 0]);
    bytes.extend(b"ABCDEFGHIJKLM   ");
    bytes.push(0xAA);
    let mut reader = BinMemoryBuffer::from(bytes);
    let vu = VuCardRecord::read(&mut reader).unwrap();
    assert_eq!(vu.card_number.identification, "ABCDEFGHIJKLM");
    assert_eq!(reader.pos().unwrap(), 45);
    assert_eq!(reader.read_u8().unwrap(), 0xAA);
}

#[test]
fn padded_numbers_parse_in_gen1_gen2_only_and_combined_downloads_with_raw_retention() {
    for (gen1, gen2) in [(true, false), (false, true), (true, true)] {
        let parsed = parse_from_memory(&card(gen1, gen2, b"ABCDEFGHIJKLM   ")).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let response = &value["cardDataResponses"];
        for (enabled, data, signature_size) in
            [(gen1, if gen2 { &response["gen1"] } else { response }, 128), (gen2, &response["gen2"], 64)]
        {
            if !enabled {
                continue;
            }
            let id = &data["identification"]["cardIdentification"];
            assert_eq!(id["cardNumber"]["identification"], "ABCDEFGHIJKLM");
            assert_eq!(id["cardExpiryDate"], json!(TimeReal::new(1_800_000_000)));
            let mut raw = identification(b"ABCDEFGHIJKLM   ");
            raw.resize(143, 0);
            assert_eq!(data["dataFiles"]["Identification"]["data"], json!(raw));
            assert_eq!(data["dataFiles"]["Identification"]["signature"], json!(vec![0x55; signature_size]));
        }
        assert!(parsed.to_xml().unwrap().contains("<identification>ABCDEFGHIJKLM</identification>"));
        assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
        let mut invalid = *b"ABCDEFGHIJKLM123";
        invalid[3] = 0xFF;
        assert!(parse_from_memory(&card(gen1, gen2, &invalid)).is_err());
    }
}

#[cfg(feature = "write")]
#[test]
fn writer_roundtrips_every_space_padded_length_for_both_card_number_layouts() {
    use esm_parser::writer::Writable;

    for kind in CARD_TYPES {
        for length in 0..=16 {
            let mut bytes = [b' '; 16];
            bytes[..length].fill(b'A');
            let number = read_number(&bytes, kind).unwrap();
            let mut writer = BinMemoryBuffer::new();
            number.write(&mut writer).unwrap();
            assert_eq!(writer.as_slice(), bytes);
            assert_eq!(read_number(writer.as_slice(), kind).unwrap(), number);
        }
    }
}
