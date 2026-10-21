use binary_data::BinMemoryBuffer;
use esm_parser::tacho::{ActivityCard, ActivityChangeInfo, ActivityChangeInfoParams, Address, HolderName, Name, OdometerShort};
use esm_parser::{BCDString, CodePage, Export, Readable, ReadableWithParams, gen1, gen2, parse_from_file, parse_from_memory};

#[test]
fn reader_root_and_generation_paths_remain_available() {
    let _ = parse_from_file;
    let _ = parse_from_memory;
    let _ = esm_parser::verify_card;
    let _ = esm_parser::verify_card_with_time;
    let _ = esm_parser::verify_vu_full;
    let _ = gen1::verify;
    let _ = gen2::verify;
    assert_eq!(BCDString::decode(&[0x12, 0x34]).unwrap(), "1234");
    assert_eq!(BCDString::decode_strict::<4>(&[0x12, 0x34]).unwrap(), "1234");
    assert_eq!(esm_parser::bytes_to_string(&[0xFC], &CodePage::IsoIec8859_1), "ü");
    assert_eq!(esm_parser::bytes_to_ia5_fix_string(b"AB  ").unwrap(), "AB");
}

#[test]
fn reader_primitives_decode_without_a_writer() {
    let mut bytes = vec![1];
    bytes.extend_from_slice(b"Musterfrau");
    bytes.resize(36, b' ');
    let name = Name::read(&mut BinMemoryBuffer::from(bytes.clone())).unwrap();
    assert_eq!(name.name, "Musterfrau");
    let address = Address::read(&mut BinMemoryBuffer::from(bytes.clone())).unwrap();
    assert_eq!(address.name, "Musterfrau");
    bytes.extend_from_within(..);
    let holder = HolderName::read(&mut BinMemoryBuffer::from(bytes)).unwrap();
    assert_eq!(holder.holder_surname, name);
    assert_eq!(holder.holder_first_names, name);

    let odometer = OdometerShort::read(&mut BinMemoryBuffer::from(vec![0x01, 0xE2, 0x40])).unwrap();
    assert_eq!(odometer.data, Some(123456));
    let unavailable = OdometerShort::read(&mut BinMemoryBuffer::from(vec![0xFF; 3])).unwrap();
    assert_eq!(unavailable.data, None);
    assert_eq!(unavailable.to_json().unwrap(), "null");
    let params = ActivityChangeInfoParams::new(ActivityCard::Card);
    let activity = ActivityChangeInfo::read(&mut BinMemoryBuffer::from(vec![0xD9, 0xE0]), &params).unwrap();
    assert_eq!(activity.activity_info, 0xD9E0);
    assert_eq!(activity.time_in_min, 480);
}

#[cfg(feature = "typescript")]
#[test]
fn reader_wasm_declarations_match_the_checked_in_contract() {
    assert_eq!(esm_parser::wasm_boundary_declaration(), include_str!("../types/wasm-boundary.d.ts"));
}
