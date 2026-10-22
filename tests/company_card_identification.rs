use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use esm_parser::tacho::{EquipmentType, Identification, IdentificationParams};
use esm_parser::{Export, ReadableWithParams, SerializedTachographData, parse_from_memory};
use serde_json::json;

fn identification(kind: EquipmentType) -> Vec<u8> {
    let mut bytes = vec![13];
    bytes.extend(b"ABCDEFGHIJKLM123");
    bytes.push(1);
    bytes.extend([b' '; 35]);
    for time in [1_700_000_000_u32, 1_700_000_100, 1_800_000_000] {
        bytes.extend(time.to_be_bytes());
    }
    let holder_size = match kind {
        EquipmentType::DriverCard => 78,
        EquipmentType::CompanyCard => 74,
        _ => 146,
    };
    bytes.resize(65 + holder_size, 0);
    bytes
}

fn section(bytes: &mut Vec<u8>, id: u16, appendix: u8, payload: &[u8]) {
    bytes.extend(id.to_be_bytes());
    bytes.push(appendix);
    bytes.extend(u16::try_from(payload.len()).unwrap().to_be_bytes());
    bytes.extend(payload);
}

fn card(gen1: bool, gen2: bool) -> Vec<u8> {
    // Partial company downloads exercise the shared identification caller in both applications.
    let mut bytes = Vec::new();
    section(&mut bytes, 0x0002, 0, &[0; 25]);
    section(&mut bytes, 0x0005, 0, &[0; 8]);
    for (enabled, appendix, signature_size) in [(gen1, 0, 128), (gen2, 2, 64)] {
        if !enabled {
            continue;
        }
        section(&mut bytes, 0x0501, appendix, &[4, u8::from(appendix == 2), 0, 0, 230]);
        section(&mut bytes, 0x0520, appendix, &identification(EquipmentType::CompanyCard));
        section(&mut bytes, 0x0520, appendix + 1, &vec![0x55; signature_size]);
    }
    bytes
}

#[test]
fn every_identification_caller_reports_its_actual_card_type_and_preserves_following_fields() {
    for kind in [EquipmentType::DriverCard, EquipmentType::WorkshopCard, EquipmentType::ControlCard, EquipmentType::CompanyCard] {
        let mut bytes = identification(kind);
        let length = bytes.len();
        bytes.push(0xAA);
        let mut reader = BinMemoryBuffer::from(bytes);
        let parsed = Identification::read(&mut reader, &IdentificationParams::new(kind)).unwrap();
        let value = serde_json::to_value(parsed).unwrap();
        let number = &value["cardIdentification"]["cardNumber"];
        assert_eq!(number["equipmentType"], json!(kind));
        assert_eq!(number["number"], "ABCDEFGHIJKLM123");
        assert_eq!(number["identification"], if kind == EquipmentType::DriverCard { "ABCDEFGHIJKLM1" } else { "ABCDEFGHIJKLM" });
        assert_eq!(number["cardConsecutiveindex"], if kind == EquipmentType::DriverCard { "" } else { "1" });
        assert_eq!(number["cardReplacementindex"], "2");
        assert_eq!(number["cardRenewalindex"], "3");
        assert_eq!(reader.pos().unwrap(), length);
        assert_eq!(reader.read_u8().unwrap(), 0xAA);
    }
}

#[test]
fn company_downloads_report_company_card_in_gen1_gen2_and_combined_applications() {
    for (gen1, gen2) in [(true, false), (false, true), (true, true)] {
        let parsed = parse_from_memory(&card(gen1, gen2)).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let responses = &value["cardDataResponses"];
        for (enabled, data, signature_size) in
            [(gen1, if gen2 { &responses["gen1"] } else { responses }, 128), (gen2, &responses["gen2"], 64)]
        {
            if !enabled {
                continue;
            }
            let number = &data["identification"]["cardIdentification"]["cardNumber"];
            assert_eq!(number["equipmentType"], "CompanyCard");
            assert_eq!(number["number"], "ABCDEFGHIJKLM123");
            assert_eq!(number["identification"], "ABCDEFGHIJKLM");
            assert_eq!(number["cardConsecutiveindex"], "1");
            assert_eq!(number["cardReplacementindex"], "2");
            assert_eq!(number["cardRenewalindex"], "3");
            assert_eq!(data["dataFiles"]["Identification"]["data"], json!(identification(EquipmentType::CompanyCard)));
            assert_eq!(data["dataFiles"]["Identification"]["signature"], json!(vec![0x55; signature_size]));
        }
        assert!(parsed.to_xml().unwrap().contains("<equipmentType>CompanyCard</equipmentType>"));
        assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
    }
}
