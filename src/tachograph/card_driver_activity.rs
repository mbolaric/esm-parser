use binary_data::{BigEndian, BinRingMemoryBuffer, BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::{ActivityCard, ActivityChangeInfo, ActivityChangeInfoParams, TimeReal};
use crate::{BCDString, Error, Readable, ReadableWithParams, Result};

/// Information, stored in a card, related to the driver activities for a
/// particular calendar day. This data type is related to Annex 1C
/// requirements 266, 291, 320 and 343.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardActivityDailyRecord {
    #[serde(rename = "activityPreviousRecordLength")]
    pub activity_previous_record_length: u16,
    #[serde(rename = "activityRecordLength")]
    pub activity_record_length: u16,
    #[serde(rename = "activityRecordDate")]
    pub activity_record_date: TimeReal,
    #[serde(rename = "activityDailyPresenceCounter")]
    pub activity_daily_presence_counter: String,
    #[serde(rename = "activityDayDistance")]
    pub activity_day_distance: u16,
    #[serde(rename = "activityChangeInfo")]
    pub activity_change_info: Vec<ActivityChangeInfo>,
}

impl Readable for CardActivityDailyRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardActivityDailyRecord> {
        let position = reader.pos()?;
        let reader_length = reader.len()?;
        let activity_previous_record_length = reader.read_u16::<BigEndian>()?;
        let activity_record_length = reader.read_u16::<BigEndian>()?;
        if activity_record_length % 2 != 0 {
            return Err(Error::CardActivityDailyRecord("Card Activity Record Length is not even".to_owned()));
        }
        let record_date = TimeReal::read(reader)?;
        let daily_presence_counter = BCDString::decode(&reader.read_into_vec(2)?)?;
        let day_distance = reader.read_u16::<BigEndian>()?;

        if activity_record_length == 0 {
            return Ok(Self {
                activity_previous_record_length,
                activity_record_length,
                activity_record_date: record_date,
                activity_daily_presence_counter: daily_presence_counter,
                activity_day_distance: day_distance,
                activity_change_info: Vec::new(),
            });
        }

        let mut end_pos = position + activity_record_length as usize;
        if end_pos >= reader_length {
            end_pos -= reader_length;
        }
        let mut activity_infos: Vec<ActivityChangeInfo> = Vec::new();
        let mut activity_counter = 0;
        let params = ActivityChangeInfoParams::new(ActivityCard::Card);
        while reader.pos()? != end_pos {
            let activity_info = ActivityChangeInfo::read(reader, &params)?;
            activity_infos.push(activity_info);
            activity_counter += 1;
            if activity_counter > 1440 {
                return Err(Error::CardActivityDailyRecord(
                    "Card with ActivityDailyRecord has more than 1440 activities in day".to_owned(),
                ));
            }
        }
        Ok(Self {
            activity_previous_record_length,
            activity_record_length,
            activity_record_date: record_date,
            activity_daily_presence_counter: daily_presence_counter,
            activity_day_distance: day_distance,
            activity_change_info: activity_infos,
        })
    }
}

#[derive(Debug)]
pub struct CardDriverActivityParams {
    pub card_activity_length_range: u32,
}

impl CardDriverActivityParams {
    pub fn new(card_activity_length_range: u32) -> Self {
        Self { card_activity_length_range }
    }
}

/// Information, stored in a driver or a workshop card, related to the
/// activities of the driver
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardDriverActivity {
    #[serde(rename = "activityPointerOldestDayRecord")]
    pub activity_pointer_oldest_day_record: u32,
    #[serde(rename = "activityPointerNewestRecord")]
    pub activity_pointer_newest_record: u32,
    #[serde(rename = "activityDailyRecords")]
    pub activity_daily_records: Vec<CardActivityDailyRecord>,
}

impl ReadableWithParams for CardDriverActivity {
    type P = CardDriverActivityParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardDriverActivity> {
        let card_activity_length_range = params.card_activity_length_range;
        let activity_pointer_oldest_day_record = reader.read_u16::<BigEndian>()? as u32;
        let activity_pointer_newest_record = reader.read_u16::<BigEndian>()? as u32;
        let activity_daily_records_raw = reader.read_into_vec(card_activity_length_range)?;

        if activity_pointer_oldest_day_record >= card_activity_length_range {
            return Err(Error::RecordOutOfRange("Oldest Day Record".to_owned()));
        }
        if activity_pointer_newest_record >= card_activity_length_range {
            return Err(Error::RecordOutOfRange("Newest Day Record".to_owned()));
        }

        let mut daily_records: Vec<CardActivityDailyRecord> = Vec::new();
        let mut activity_reader =
            BinRingMemoryBuffer::new_with_offset(activity_daily_records_raw, activity_pointer_oldest_day_record as usize);

        loop {
            let position = activity_reader.pos()?;
            let record = CardActivityDailyRecord::read(&mut activity_reader)?;
            daily_records.push(record);

            if position == activity_pointer_newest_record as usize {
                break;
            }
        }
        identify_card_withdrawals(&mut daily_records);

        Ok(Self { activity_pointer_oldest_day_record, activity_pointer_newest_record, activity_daily_records: daily_records })
    }
}

fn identify_card_withdrawals(daily_records: &mut [CardActivityDailyRecord]) {
    const SECONDS_PER_DAY: u32 = 86_400;

    let mut previous_date: Option<u32> = None;
    let mut previous_card_inserted: Option<bool> = None;
    for daily_record in daily_records {
        let current_date = daily_record.activity_record_date.get_data();
        let initial_card_inserted = match previous_date {
            Some(date) if date.checked_add(SECONDS_PER_DAY) == Some(current_date) => previous_card_inserted,
            _ => None,
        };
        previous_card_inserted =
            ActivityChangeInfo::identify_card_withdrawals(&mut daily_record.activity_change_info, initial_card_inserted);
        previous_date = Some(current_date);
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinRingMemoryBuffer;

    use super::*;
    use crate::tacho::{ActivityType, CardSlotNumber};

    fn activity_word(slot: u16, status: u16, card_status: u16, activity: u16, minutes: u16) -> u16 {
        (slot << 15) | (status << 14) | (card_status << 13) | (activity << 11) | minutes
    }

    #[test]
    fn identifies_a_withdrawal_while_reading_a_complete_card_day() {
        let mut data = Vec::new();
        data.extend_from_slice(&0_u16.to_be_bytes());
        data.extend_from_slice(&16_u16.to_be_bytes());
        data.extend_from_slice(&1_u32.to_be_bytes());
        data.extend_from_slice(&[0x00, 0x01]);
        data.extend_from_slice(&0_u16.to_be_bytes());
        data.extend_from_slice(&activity_word(1, 0, 0, 2, 480).to_be_bytes());
        data.extend_from_slice(&activity_word(1, 0, 1, 3, 600).to_be_bytes());
        let mut reader = BinRingMemoryBuffer::new_with_offset(data, 0);

        let record = CardActivityDailyRecord::read(&mut reader).expect("the card day must parse");
        let mut records = vec![record];
        identify_card_withdrawals(&mut records);

        let withdrawal = &records[0].activity_change_info[1];
        assert!(withdrawal.is_card_withdrawal);
        assert_eq!(withdrawal.card_slot, CardSlotNumber::CoDriver);
        assert_eq!(withdrawal.encoded_activity_type, ActivityType::Driving);
        assert_eq!(withdrawal.activity_type, ActivityType::Unknown);
    }

    #[test]
    fn identifies_a_withdrawal_at_midnight_from_the_previous_day_state() {
        let first_day = CardActivityDailyRecord {
            activity_previous_record_length: 0,
            activity_record_length: 0,
            activity_record_date: TimeReal::new(86_400),
            activity_daily_presence_counter: "0001".to_owned(),
            activity_day_distance: 0,
            activity_change_info: vec![ActivityChangeInfo::new(ActivityCard::Card, activity_word(0, 0, 0, 2, 480))],
        };
        let second_day = CardActivityDailyRecord {
            activity_previous_record_length: 0,
            activity_record_length: 0,
            activity_record_date: TimeReal::new(172_800),
            activity_daily_presence_counter: "0002".to_owned(),
            activity_day_distance: 0,
            activity_change_info: vec![ActivityChangeInfo::new(ActivityCard::Card, activity_word(0, 0, 1, 2, 0))],
        };
        let mut records = vec![first_day, second_day];

        identify_card_withdrawals(&mut records);

        assert!(records[1].activity_change_info[0].is_card_withdrawal);
        assert_eq!(records[1].activity_change_info[0].card_slot, CardSlotNumber::Driver);
    }
}
