#[path = "common/card.rs"]
mod card;

use binary_data::{BinMemoryBuffer, BinRingMemoryBuffer};
use card::section;
use esm_parser::gen2::{SpecificConditions, SpecificConditionsParams};
use esm_parser::tacho::{CardActivityDailyRecord, CardDriverActivity, CardDriverActivityParams};
use esm_parser::{Error, Readable, ReadableWithParams, parse_from_memory};

type Section = (u16, u8, Vec<u8>);

fn files(kind: u8, gen1: bool, gen2: bool) -> Vec<Section> {
    let mut files = vec![(2, 0, vec![0; 25]), (5, 0, vec![0; 8])];
    for (enabled, appendix) in [(gen1, 0), (gen2, 2)] {
        if !enabled {
            continue;
        }
        let mut app = vec![kind, u8::from(appendix == 2), 0];
        if kind >= 3 {
            app.extend(230_u16.to_be_bytes());
        } else {
            app.extend(match (kind, appendix) {
                (2, 0) => [3, 1],
                (2, _) => [3, 6],
                (_, 0) => [6, 12],
                _ => [12, 24],
            });
            if appendix == 0 {
                for count in if kind == 2 { [492_u16, 4] } else { [13776, 200] } {
                    app.extend(count.to_be_bytes());
                }
                app.push(if kind == 2 { 3 } else { 112 });
                if kind == 2 {
                    app.push(88);
                }
            } else {
                for count in if kind == 2 { vec![492_u16, 8, 8, 255, 24, 4, 8] } else { vec![13776, 200, 112, 336, 112, 200] } {
                    app.extend(count.to_be_bytes());
                }
            }
        }
        files.push((0x0501, appendix, app));
    }
    files
}

fn download(files: &[Section]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (id, appendix, payload) in files {
        section(&mut bytes, *id, *appendix, payload);
    }
    bytes
}

#[test]
fn all_card_types_reject_extra_bytes_in_required_and_known_optional_efs() {
    for kind in 1..=4 {
        for (gen1, gen2) in [(true, false), (false, true), (true, true)] {
            let original = files(kind, gen1, gen2);
            assert!(parse_from_memory(&download(&original)).is_ok());
            for index in 0..original.len() {
                let mut malformed = original.clone();
                malformed[index].2.push(0xAA);
                assert!(parse_from_memory(&download(&malformed)).is_err(), "card {kind}, EF {}", original[index].0);
            }
            for (enabled, appendix) in [(gen1, 0), (gen2, 2)] {
                if !enabled {
                    continue;
                }
                let size = match kind {
                    1 => 143,
                    4 => 139,
                    _ => 211,
                };
                let mut valid = original.clone();
                valid.push((0x0520, appendix, vec![0; size]));
                assert!(parse_from_memory(&download(&valid)).is_ok());
                for length in [size - 1, size + 1] {
                    let mut malformed = original.clone();
                    malformed.push((0x0520, appendix, vec![0; length]));
                    assert!(parse_from_memory(&download(&malformed)).is_err());
                }
            }
        }
    }
}

#[test]
fn application_v2_lengths_are_checked_for_every_card_type() {
    for kind in 1..=4 {
        for gen1 in [false, true] {
            let expected = if kind <= 2 { 8_u16 } else { 2 };
            let mut prefix = expected.to_be_bytes().to_vec();
            prefix.extend([0; 8].into_iter().take(usize::from(expected)));
            let mut valid = files(kind, gen1, true);
            valid.push((0x0525, 2, prefix.clone()));
            assert!(parse_from_memory(&download(&valid)).is_ok());
            for (declared, actual) in
                [(expected - 1, expected), (expected + 1, expected + 1), (expected, expected - 1), (expected, expected + 1)]
            {
                let mut payload = declared.to_be_bytes().to_vec();
                payload.resize(usize::from(actual) + 2, 0);
                let mut malformed = files(kind, gen1, true);
                malformed.push((0x0525, 2, payload));
                assert!(parse_from_memory(&download(&malformed)).is_err(), "card {kind}, declared {declared}, actual {actual}");
            }
        }
    }
}

#[test]
fn specific_condition_capacities_are_not_narrowed_and_pointers_and_ef_sizes_are_checked() {
    for kind in [1, 2] {
        for gen1 in [false, true] {
            for count in [0_u16, 4, 56, 112, 113, 256] {
                let mut entries = files(kind, gen1, true);
                let app = &mut entries.iter_mut().find(|(id, appendix, _)| *id == 0x0501 && *appendix == 2).unwrap().2;
                let offset = if kind == 2 { 15 } else { 13 };
                app[offset..offset + 2].copy_from_slice(&count.to_be_bytes());
                let mut payload = count.saturating_sub(1).to_be_bytes().to_vec();
                for index in 0..count {
                    payload.extend((1_700_000_000 + u32::from(index)).to_be_bytes());
                    payload.push(1);
                }
                entries.push((0x0522, 2, payload.clone()));
                let valid_capacity = (1..=if kind == 2 { 4 } else { 112 }).contains(&count);
                assert_eq!(parse_from_memory(&download(&entries)).is_ok(), valid_capacity);
                if valid_capacity {
                    for length in [payload.len() - 1, payload.len() + 1] {
                        let mut malformed = entries.clone();
                        malformed.last_mut().unwrap().2.resize(length, 0);
                        assert!(parse_from_memory(&download(&malformed)).is_err());
                    }
                    let payload = &mut entries.last_mut().unwrap().2;
                    payload[..2].copy_from_slice(&count.to_be_bytes());
                    assert!(parse_from_memory(&download(&entries)).is_err());
                }
            }
        }
    }
    for capacity in [0, 113, 256, u32::MAX] {
        assert!(
            SpecificConditions::read(&mut BinMemoryBuffer::from(vec![0; 2]), &SpecificConditionsParams::new(capacity)).is_err()
        );
    }
}

#[test]
fn signature_size_checks_preserve_unsigned_and_cryptographically_invalid_downloads() {
    for (appendix, valid_sizes, invalid_sizes) in [(0, vec![128], vec![0, 64, 127, 129]), (2, vec![64, 132], vec![0, 63, 133])] {
        for size in valid_sizes {
            let mut entries = files(1, appendix == 0, appendix == 2);
            entries.push((0x0501, appendix + 1, vec![0x55; size]));
            assert!(parse_from_memory(&download(&entries)).is_ok());
        }
        for size in invalid_sizes {
            let mut entries = files(1, appendix == 0, appendix == 2);
            entries.push((0x0501, appendix + 1, vec![0x55; size]));
            assert!(parse_from_memory(&download(&entries)).is_err());
        }
    }
}

#[test]
fn activity_record_lengths_and_ring_traversal_cannot_escape_the_activity_buffer() {
    for length in [2_u16, 10, 14, u16::MAX] {
        let mut raw = vec![0; 12];
        raw[2..4].copy_from_slice(&length.to_be_bytes());
        assert!(CardActivityDailyRecord::read(&mut BinRingMemoryBuffer::new_with_offset(raw, 0)).is_err());
    }
    let mut valid = vec![0; 16];
    assert!(CardDriverActivity::read(&mut BinMemoryBuffer::from(valid.clone()), &CardDriverActivityParams::new(12)).is_ok());
    valid[3] = 1;
    assert!(CardDriverActivity::read(&mut BinMemoryBuffer::from(valid), &CardDriverActivityParams::new(12)).is_err());
    for capacity in [0, 11, u32::MAX] {
        assert!(
            CardDriverActivity::read(&mut BinMemoryBuffer::from(vec![0; 16]), &CardDriverActivityParams::new(capacity)).is_err()
        );
    }
}

#[test]
fn every_card_ring_rejects_an_out_of_range_newest_record_pointer() {
    use esm_parser::gen1::{CardVehicleRecord, PlaceRecord};
    use esm_parser::gen2::{
        CardVehicleUnitsUsed, CardVehicleUnitsUsedParams, GnssAccumulatedDriving, GnssAccumulatedDrivingParams, LoadTypeEntries,
        LoadTypeEntriesParams,
    };
    use esm_parser::tacho::{
        CardPlaceDailyWorkPeriod, CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, CompanyActivityData,
        CompanyActivityDataParams, ControlCardControlActivityData, ControlCardControlActivityDataParams, TimeReal,
        VehiclesUsedParams,
    };

    macro_rules! rejects {
        ($record:ty, $params:expr) => {
            assert!(matches!(<$record>::read(&mut BinMemoryBuffer::from(vec![0, 2]), &$params), Err(Error::RecordOutOfRange(_))));
        };
    }
    rejects!(CardVehiclesUsed<CardVehicleRecord>, VehiclesUsedParams::new(2));
    rejects!(CardPlaceDailyWorkPeriod<PlaceRecord>, CardPlaceDailyWorkPeriodParams::new(2, 2));
    rejects!(CompanyActivityData<TimeReal>, CompanyActivityDataParams::new(2));
    rejects!(ControlCardControlActivityData<TimeReal>, ControlCardControlActivityDataParams::new(2));
    rejects!(CardVehicleUnitsUsed, CardVehicleUnitsUsedParams::new(2));
    rejects!(GnssAccumulatedDriving, GnssAccumulatedDrivingParams::new(2));
    rejects!(LoadTypeEntries, LoadTypeEntriesParams::new(2));
}
