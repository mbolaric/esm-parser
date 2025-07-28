use binary_data::{BigEndian, BinRingMemoryBuffer, BinSeek, ReadBytes};

use crate::{
    BCDString, Error, Readable, ReadableWithParams, Result,
    tacho::{ActivityCard, ActivityChangeInfo, ActivityChangeInfoParams, TimeReal},
};

#[derive(Debug)]
pub struct CardActivityDailyRecord {
    pub record_date: TimeReal,
    pub daily_presence_counter: String,
    pub day_distance: u16,
    pub activity_infos: Vec<ActivityChangeInfo>,
}

impl Readable<CardActivityDailyRecord> for CardActivityDailyRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardActivityDailyRecord> {
        let position = reader.pos()?;
        let reader_length = reader.len()?;
        let _activity_previous_record_length = reader.read_u16::<BigEndian>()?;
        let activity_record_length = reader.read_u16::<BigEndian>()?;
        if activity_record_length % 2 != 0 {
            return Err(Error::CardActivityDailyRecord("Card Activity Record Length is not even".to_owned()));
        }
        let record_date = TimeReal::read(reader)?;
        let daily_presence_counter = BCDString::decode(&reader.read_into_vec(2)?)?;
        let day_distance = reader.read_u16::<BigEndian>()?;

        if activity_record_length == 0 {
            return Ok(Self { record_date, daily_presence_counter, day_distance, activity_infos: Vec::new() });
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

        Ok(Self { record_date, daily_presence_counter, day_distance, activity_infos })
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

#[derive(Debug)]
pub struct CardDriverActivity {
    pub activity_daily_records: Vec<CardActivityDailyRecord>,
}

impl ReadableWithParams<CardDriverActivity> for CardDriverActivity {
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

        Ok(Self { activity_daily_records: daily_records })
    }
}
