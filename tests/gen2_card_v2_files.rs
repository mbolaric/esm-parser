use binary_data::{BinMemoryBuffer, BinSeek};
use esm_parser::gen2::{CardAuthenticationRecordsParams, CardPlaceAuthDailyWorkPeriod, GnssAuthAccumulatedDriving};
use esm_parser::tacho::{CardFileID, EquipmentType};
use esm_parser::{Export, ReadableWithParams, SerializedTachographData, parse_from_memory};
use serde_json::Value;

fn section(bytes: &mut Vec<u8>, id: u16, appendix: u8, payload: &[u8]) {
    bytes.extend(id.to_be_bytes());
    bytes.push(appendix);
    bytes.extend(u16::try_from(payload.len()).unwrap().to_be_bytes());
    bytes.extend(payload);
}

fn ring(pointer: u16, count: usize, size: usize, first: &[u8], last: &[u8]) -> Vec<u8> {
    assert_eq!(first.len(), size);
    assert_eq!(last.len(), size);
    let mut bytes = pointer.to_be_bytes().to_vec();
    bytes.resize(2 + count * size, 0);
    bytes[2..2 + size].copy_from_slice(first);
    bytes[2 + (count - 1) * size..].copy_from_slice(last);
    bytes
}

fn timestamped(time: u32, suffix: &[u8]) -> Vec<u8> {
    [time.to_be_bytes().as_slice(), suffix].concat()
}

fn card(kind: EquipmentType, combined: bool) -> Vec<u8> {
    let workshop = kind == EquipmentType::WorkshopCard;
    let (places, gnss, loads) = if workshop { (8, 24, 4) } else { (112, 336, 336) };
    let mut bytes = Vec::new();
    section(&mut bytes, 0x0002, 0, &[0; 25]);
    section(&mut bytes, 0x0005, 0, &[0; 8]);
    if combined {
        let mut app = vec![kind as u8, 0, 0, 6, 2];
        app.extend(13776_u16.to_be_bytes());
        app.extend(200_u16.to_be_bytes());
        app.push(if workshop { 3 } else { 112 });
        if workshop {
            app.push(88);
        }
        section(&mut bytes, 0x0501, 0, &app);
    }
    let mut app = vec![kind as u8, 1, 1, if workshop { 3 } else { 12 }, if workshop { 6 } else { 24 }];
    let capacities = if workshop { vec![492_u16, 8, 8, 255, 24, 4, 8] } else { vec![13776_u16, 200, 112, 336, 112, 200] };
    for capacity in capacities {
        app.extend(capacity.to_be_bytes());
    }
    section(&mut bytes, 0x0501, 2, &app);
    let mut app_v2 = Vec::new();
    for capacity in [8_u16, if workshop { 4 } else { 1120 }, if workshop { 8 } else { 1624 }, loads as u16, 3072] {
        app_v2.extend(capacity.to_be_bytes());
    }
    section(&mut bytes, 0x0525, 2, &app_v2);

    let mut place_suffix = vec![0, 13, 0, 0, 0, 1];
    place_suffix.extend(timestamped(90, &[1, 0, 0, 0, 0, 0, 0]));
    section(&mut bytes, 0x0506, 2, &ring(1, places, 21, &timestamped(100, &place_suffix), &timestamped(200, &place_suffix)));
    let mut gnss_suffix = timestamped(990, &[1, 0, 0, 0, 0, 0, 0]);
    gnss_suffix.extend([0, 0, 1]);
    section(&mut bytes, 0x0524, 2, &ring(1, gnss, 18, &timestamped(1000, &gnss_suffix), &timestamped(2000, &gnss_suffix)));

    let place_auth = ring((places - 1) as u16, places, 5, &timestamped(200, &[0]), &timestamped(100, &[1]));
    let gnss_auth = ring((gnss - 1) as u16, gnss, 5, &timestamped(2000, &[0]), &timestamped(1000, &[1]));
    let load_entries = ring((loads - 1) as u16, loads, 5, &timestamped(3000, &[1]), &timestamped(4000, &[2]));
    for (id, payload) in [(0x0526, place_auth), (0x0527, gnss_auth), (0x0530, load_entries)] {
        section(&mut bytes, id, 2, &payload);
        section(&mut bytes, id, 3, &[id as u8; 64]);
    }
    bytes
}

fn without_files(bytes: &[u8], ids: &[u16]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < bytes.len() {
        let id = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        let length = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]) as usize;
        let end = offset + 5 + length;
        if !ids.contains(&id) {
            result.extend_from_slice(&bytes[offset..end]);
        }
        offset = end;
    }
    result
}

fn assert_authentication(auth: &Value, pointer: &str, records: &str, time: &str, count: usize, first: u32, last: u32) {
    assert_eq!(auth[pointer], count - 1);
    let records = auth[records].as_array().unwrap();
    assert_eq!(records.len(), count);
    assert_eq!(records[0][time], serde_json::to_value(esm_parser::tacho::TimeReal::new(first)).unwrap());
    assert_eq!(records[0]["authenticationStatus"], 0);
    assert_eq!(records[1]["authenticationStatus"], 0);
    assert_eq!(records[count - 1][time], serde_json::to_value(esm_parser::tacho::TimeReal::new(last)).unwrap());
    assert_eq!(records[count - 1]["authenticationStatus"], 1);
}

#[test]
fn recognizes_the_normative_v2_file_identifiers() {
    for (id, name) in [(0x0526, "PlacesAuthentication"), (0x0527, "GnssPlacesAuthentication"), (0x0530, "LoadTypeEntries")] {
        let parsed = CardFileID::from(id);
        assert_eq!(parsed.to_string(), name);
        assert_eq!(parsed as u16, id);
    }
    assert_eq!(CardFileID::from(0x052A), CardFileID::Unknown);
}

#[test]
fn load_type_entries_require_v2_capacities_in_both_card_types() {
    for kind in [EquipmentType::DriverCard, EquipmentType::WorkshopCard] {
        let bytes = without_files(&card(kind, false), &[0x0525]);
        let error = parse_from_memory(&bytes).unwrap_err();
        assert!(matches!(error, esm_parser::Error::MissingCardFile(message)
            if message.contains("ApplicationIdentificationV2") && message.contains("LoadTypeEntries")));
    }
}

#[test]
fn cards_without_the_new_optional_files_still_parse() {
    for kind in [EquipmentType::DriverCard, EquipmentType::WorkshopCard] {
        let bytes = without_files(&card(kind, false), &[0x0525, 0x0526, 0x0527, 0x0530]);
        let parsed = serde_json::to_value(parse_from_memory(&bytes).unwrap()).unwrap();
        let gen2 = &parsed["cardDataResponses"]["gen2"];
        assert!(gen2["placesAuthentication"].is_null());
        assert!(gen2["gnssPlacesAuthentication"].is_null());
        assert!(gen2["loadTypeEntries"].is_null());
    }
}

#[test]
fn authentication_readers_reject_invalid_capacities_lengths_and_pointers() {
    let valid = ring(1, 2, 5, &timestamped(1, &[0]), &timestamped(2, &[1]));
    let mut cases = vec![(valid.clone(), 2, true), (vec![0, 0], 0, false), (vec![0, 0], 65536, false)];
    for length in [0, 1, 2, 6, 11, 13] {
        let mut bytes = valid.clone();
        bytes.resize(length, 0);
        cases.push((bytes, 2, false));
    }
    let mut bad_pointer = valid;
    bad_pointer[..2].copy_from_slice(&2_u16.to_be_bytes());
    cases.push((bad_pointer, 2, false));
    for (bytes, count, expected) in cases {
        let params = CardAuthenticationRecordsParams::new(count);
        assert_eq!(CardPlaceAuthDailyWorkPeriod::read(&mut BinMemoryBuffer::from(bytes.clone()), &params).is_ok(), expected);
        assert_eq!(GnssAuthAccumulatedDriving::read(&mut BinMemoryBuffer::from(bytes), &params).is_ok(), expected);
    }
}

#[test]
fn authentication_readers_preserve_empty_slots_and_unmapped_status_codes() {
    let bytes = ring(1, 2, 5, &[0; 5], &timestamped(42, &[0xFE]));
    let params = CardAuthenticationRecordsParams::new(2);
    let mut reader = BinMemoryBuffer::from(bytes.clone());
    let places = CardPlaceAuthDailyWorkPeriod::read(&mut reader, &params).unwrap();
    assert_eq!(reader.pos().unwrap(), bytes.len());
    assert_eq!(places.place_auth_status_records.len(), 2);
    assert_eq!(places.place_auth_status_records[0].entry_time.data, 0);
    assert_eq!(places.place_auth_status_records[1].entry_time.data, 42);
    assert_eq!(places.place_auth_status_records[1].authentication_status, 0xFE);

    let mut reader = BinMemoryBuffer::from(bytes.clone());
    let gnss = GnssAuthAccumulatedDriving::read(&mut reader, &params).unwrap();
    assert_eq!(reader.pos().unwrap(), bytes.len());
    assert_eq!(gnss.gnss_auth_status_ad_records.len(), 2);
    assert_eq!(gnss.gnss_auth_status_ad_records[0].time_stamp.data, 0);
    assert_eq!(gnss.gnss_auth_status_ad_records[1].time_stamp.data, 42);
    assert_eq!(gnss.gnss_auth_status_ad_records[1].authentication_status, 0xFE);
}

#[test]
fn imports_v2_files_in_driver_and_workshop_cards_with_both_generation_modes() {
    for kind in [EquipmentType::DriverCard, EquipmentType::WorkshopCard] {
        for combined in [false, true] {
            let bytes = card(kind, combined);
            let parsed = parse_from_memory(&bytes).unwrap();
            let json = serde_json::to_value(&parsed).unwrap();
            let gen2 = &json["cardDataResponses"]["gen2"];
            let (places, gnss) = if kind == EquipmentType::WorkshopCard { (8, 24) } else { (112, 336) };
            assert_authentication(
                &gen2["placesAuthentication"],
                "placeAuthPointerNewestRecord",
                "placeAuthStatusRecords",
                "entryTime",
                places,
                200,
                100,
            );
            assert_authentication(
                &gen2["gnssPlacesAuthentication"],
                "gnssAuthADPointerNewestRecord",
                "gnssAuthStatusADRecords",
                "timeStamp",
                gnss,
                2000,
                1000,
            );
            assert_eq!(gen2["loadTypeEntries"]["cardLoadTypeEntryRecords"][0]["loadTypeEntered"], "Goods");
            assert_eq!(gen2["loadTypeEntries"]["cardLoadTypeEntryRecords"][1]["loadTypeEntered"], "Passengers");
            assert_eq!(
                gen2["places"]["placeRecords"][0]["entryTime"],
                gen2["placesAuthentication"]["placeAuthStatusRecords"][places - 1]["entryTime"]
            );
            assert_eq!(
                gen2["gnssPlaces"]["gnssAccumulatedDrivingRecords"][0]["timeStamp"],
                gen2["gnssPlacesAuthentication"]["gnssAuthStatusADRecords"][gnss - 1]["timeStamp"]
            );
            for (id, name) in
                [(0x0526_u16, "PlacesAuthentication"), (0x0527, "GnssPlacesAuthentication"), (0x0530, "LoadTypeEntries")]
            {
                assert_eq!(gen2["dataFiles"][name]["signature"], serde_json::json!(vec![id as u8; 64]));
            }
            assert_eq!(json["cardDataResponses"].get("gen1").is_some(), combined);
            let xml = parsed.to_xml().unwrap();
            assert!(xml.contains("<placesAuthentication>"));
            assert!(xml.contains("<gnssPlacesAuthentication>"));
            let envelope = serde_json::to_value(SerializedTachographData::from(parsed)).unwrap();
            assert_eq!(envelope["kind"], "cardGen2");
            assert_eq!(envelope["data"]["cardDataResponses"]["gen2"], *gen2);
        }
    }
}

#[test]
fn imports_normative_border_crossings_in_gen2_only_and_combined_driver_cards() {
    let first = [13, 17, 0, 0, 0, 100, 1, 0, 0xB9, 0x94, 0, 0x0D, 0x06, 0, 1, 0xE2, 0x40];
    let last = [17, 13, 0, 0, 0, 200, 7, 0, 0x0D, 0x06, 0, 0xB9, 0x94, 1, 9, 0xFB, 0xF1];
    let payload = ring(1119, 1120, 17, &first, &last);
    let signature = [0x52; 64];
    for combined in [false, true] {
        let mut bytes = card(EquipmentType::DriverCard, combined);
        section(&mut bytes, 0x0528, 2, &payload);
        section(&mut bytes, 0x0528, 3, &signature);
        let parsed = parse_from_memory(&bytes).unwrap();
        let json = serde_json::to_value(&parsed).unwrap();
        let gen2 = &json["cardDataResponses"]["gen2"];
        let borders = &gen2["borderCrossings"];
        assert_eq!(borders["borderCrossingPointerNewestRecord"], 1119);
        let records = borders["cardBorderCrossingRecords"].as_array().unwrap();
        assert_eq!(records.len(), 2);
        for (record, left, entered, timestamp, status, odometer) in
            [(&records[0], "Germany", "France", 100, 0, 123456), (&records[1], "France", "Germany", 200, 1, 654321)]
        {
            assert_eq!(record["countryLeft"], left);
            assert_eq!(record["countryEntered"], entered);
            assert_eq!(
                record["gnssPlaceAuthRecord"]["timeStamp"],
                serde_json::json!(esm_parser::tacho::TimeReal::new(timestamp))
            );
            assert_eq!(record["gnssPlaceAuthRecord"]["authenticationStatus"], status);
            assert_eq!(record["vehicleOdometerValue"], odometer);
            assert!(record.get("timeStamp").is_none());
            assert!(record.get("gnssPlaceRecord").is_none());
        }
        assert_eq!(gen2["dataFiles"]["BorderCrossings"]["data"], serde_json::json!(payload));
        assert_eq!(gen2["dataFiles"]["BorderCrossings"]["signature"], serde_json::json!(signature.as_slice()));
        assert_eq!(json["cardDataResponses"].get("gen1").is_some(), combined);
        assert!(parsed.to_xml().unwrap().contains("<gnssPlaceAuthRecord>"));
        let envelope = serde_json::to_value(SerializedTachographData::from(parsed)).unwrap();
        assert_eq!(envelope["data"]["cardDataResponses"]["gen2"]["borderCrossings"], *borders);
    }
}

#[test]
fn border_crossings_reject_malformed_efs_through_the_public_parser() {
    let valid = ring(0, 1120, 17, &[0; 17], &[0; 17]);
    let mut cases = Vec::new();
    for length in [valid.len() - 1, valid.len() + 1, 2 + 1120 * 20] {
        let mut payload = valid.clone();
        payload.resize(length, 0);
        cases.push(payload);
    }
    let mut invalid_pointer = valid.clone();
    invalid_pointer[..2].copy_from_slice(&1120_u16.to_be_bytes());
    cases.push(invalid_pointer);
    for payload in cases {
        let mut bytes = card(EquipmentType::DriverCard, false);
        section(&mut bytes, 0x0528, 2, &payload);
        assert!(parse_from_memory(&bytes).is_err());
    }
    let mut bytes = without_files(&card(EquipmentType::DriverCard, false), &[0x0525, 0x0530]);
    section(&mut bytes, 0x0528, 2, &valid);
    assert!(matches!(parse_from_memory(&bytes), Err(esm_parser::Error::MissingCardFile(message))
        if message.contains("ApplicationIdentificationV2") && message.contains("BorderCrossings")));
}

#[test]
fn imports_load_unload_operations_in_gen2_only_and_combined_driver_cards() {
    let load = [0, 0, 0, 100, 1, 0, 0, 0, 90, 1, 0, 0xB9, 0x94, 0, 0x0D, 6, 0, 1, 0xE2, 0x40];
    let unload = [0, 0, 0, 200, 2, 0, 0, 0, 190, 7, 0, 0x0D, 6, 0, 0xB9, 0x94, 1, 9, 0xFB, 0xF1];
    let both = [0, 0, 1, 44, 3, 0, 0, 1, 34, 3, 0, 0, 0, 0, 0, 0, 1, 0, 0, 42];
    let mut payload = ring(1623, 1624, 20, &load, &both);
    payload[22..42].copy_from_slice(&unload);
    let signature = [0x53; 64];
    for combined in [false, true] {
        let mut bytes = card(EquipmentType::DriverCard, combined);
        section(&mut bytes, 0x0529, 2, &payload);
        section(&mut bytes, 0x0529, 3, &signature);
        let parsed = parse_from_memory(&bytes).unwrap();
        let json = serde_json::to_value(&parsed).unwrap();
        let gen2 = &json["cardDataResponses"]["gen2"];
        let operations = &gen2["loadUnloadOperations"];
        assert_eq!(operations["loadUnloadPointerNewestRecord"], 1623);
        let records = operations["cardLoadUnloadRecords"].as_array().unwrap();
        assert_eq!(records.len(), 3);
        for (record, operation, timestamp, status, odometer) in [
            (&records[0], "Load", 100, 0, 123456),
            (&records[1], "Unload", 200, 1, 654321),
            (&records[2], "SimultaneousLoadAndUnload", 300, 1, 42),
        ] {
            assert_eq!(record["operationType"], operation);
            assert_eq!(record["timeStamp"], serde_json::json!(esm_parser::tacho::TimeReal::new(timestamp)));
            assert_eq!(
                record["gnssPlaceAuthRecord"]["timeStamp"],
                serde_json::json!(esm_parser::tacho::TimeReal::new(timestamp - 10))
            );
            assert_eq!(record["gnssPlaceAuthRecord"]["authenticationStatus"], status);
            assert_eq!(record["vehicleOdometerValue"], odometer);
            assert!(record.get("country").is_none());
            assert!(record.get("region").is_none());
            assert!(record.get("gnssPlaceRecord").is_none());
        }
        assert_eq!(gen2["dataFiles"]["LoadUnloadOperations"]["data"], serde_json::json!(payload));
        assert_eq!(gen2["dataFiles"]["LoadUnloadOperations"]["signature"], serde_json::json!(signature.as_slice()));
        assert_eq!(json["cardDataResponses"].get("gen1").is_some(), combined);
        let xml = parsed.to_xml().unwrap();
        assert!(xml.contains("<loadUnloadOperations>"));
        assert!(xml.contains("<gnssPlaceAuthRecord>"));
        let envelope = serde_json::to_value(SerializedTachographData::from(parsed)).unwrap();
        assert_eq!(envelope["data"]["cardDataResponses"]["gen2"]["loadUnloadOperations"], *operations);
    }
}

#[test]
fn load_unload_operations_reject_malformed_efs_through_the_public_parser() {
    let valid = ring(0, 1624, 20, &[0; 20], &[0; 20]);
    let mut cases = Vec::new();
    for length in [valid.len() - 1, valid.len() + 1, 2 + 1624 * 21] {
        let mut payload = valid.clone();
        payload.resize(length, 0);
        cases.push(payload);
    }
    let mut invalid_pointer = valid.clone();
    invalid_pointer[..2].copy_from_slice(&1624_u16.to_be_bytes());
    cases.push(invalid_pointer);
    for payload in cases {
        let mut bytes = card(EquipmentType::DriverCard, false);
        section(&mut bytes, 0x0529, 2, &payload);
        assert!(parse_from_memory(&bytes).is_err());
    }
    let mut bytes = without_files(&card(EquipmentType::DriverCard, false), &[0x0525, 0x0530]);
    section(&mut bytes, 0x0529, 2, &valid);
    assert!(matches!(parse_from_memory(&bytes), Err(esm_parser::Error::MissingCardFile(message))
        if message.contains("ApplicationIdentificationV2") && message.contains("LoadUnloadOperations")));
}
