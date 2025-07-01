use binary_data::{BigEndian, BinRingMemoryBuffer, BinSeek, ReadBytes};

use crate::{
    BCDString, Error, Readable, ReadableWithParams, Result,
    tacho::{ActivitySource, ActivityType, CardSlotNumber, CardStatus, DrivingStatus, TimeReal},
};

#[derive(Debug)]
pub struct CardDriverActivityParams {
    pub card_activity_length_range: u32,
}

impl CardDriverActivityParams {
    pub fn new(card_activity_length_range: u32) -> Self {
        Self { card_activity_length_range }
    }
}

#[derive(Debug, PartialEq)]
pub enum ActivityCard {
    Vu,
    Card,
}

#[derive(Debug)]
pub struct CardDriverActivityInfo {
    pub activity_card: ActivityCard,
    pub activity_info: u16,
    pub card_status: CardStatus,
    pub driving_status: DrivingStatus,
    pub card_slot: CardSlotNumber,
    pub activity_source: ActivitySource,
    pub activity_type: ActivityType,
    pub time_in_min: u16,
}

impl CardDriverActivityInfo {
    pub fn new(activity_card: ActivityCard, activity_info: u16) -> Self {
        let card_status = (((activity_info >> 13) & 1) as u8).into();
        let driving_status = CardDriverActivityInfo::get_driving_crow(&card_status, activity_info);
        let card_slot_valid = CardDriverActivityInfo::get_slot_valid(&activity_card, &card_status);
        let card_slot = CardDriverActivityInfo::get_card_slot(card_slot_valid, activity_info);
        let activity_source = CardDriverActivityInfo::get_activity_source(&card_status, &driving_status);
        let activity_type = CardDriverActivityInfo::get_activity_type(&activity_card, &activity_source, activity_info);
        let time_in_min = activity_info & 0x7FF;

        Self { activity_card, activity_info, card_status, driving_status, card_slot, activity_source, activity_type, time_in_min }
    }

    fn get_activity_source(card_status: &CardStatus, driving_status: &DrivingStatus) -> ActivitySource {
        if *card_status == CardStatus::Removed {
            if *driving_status != DrivingStatus::SingleOrUnknown {
                return ActivitySource::Unknown;
            }
            return ActivitySource::Manual;
        }
        ActivitySource::Automatic
    }

    fn get_activity_type(activity_card: &ActivityCard, activity_source: &ActivitySource, activity_info: u16) -> ActivityType {
        if *activity_source == ActivitySource::Unknown && *activity_card == ActivityCard::Card {
            return ActivityType::Unknown;
        }
        ((activity_info >> 11) & 0b11).into()
    }

    fn get_card_slot(card_slot_valid: bool, activity_info: u16) -> CardSlotNumber {
        if !card_slot_valid {
            return CardSlotNumber::Unknown;
        }
        if (activity_info & (1 << 15)) == 0 {
            return CardSlotNumber::Driver;
        }
        CardSlotNumber::CoDriver
    }

    fn get_slot_valid(activity_card: &ActivityCard, card_status: &CardStatus) -> bool {
        if *activity_card == ActivityCard::Vu {
            return true;
        }
        *card_status == CardStatus::Inserted
    }

    fn get_driving_crow(card_status: &CardStatus, activity_info: u16) -> DrivingStatus {
        if *card_status == CardStatus::Removed {
            return DrivingStatus::SingleOrUnknown;
        }
        if *card_status == CardStatus::Inserted {
            if activity_info & (1 << 15) == 0 {
                return DrivingStatus::SingleOrUnknown;
            }
            return DrivingStatus::CrowOrKnown;
        }
        DrivingStatus::Unknown
    }
}

impl Readable<CardDriverActivityInfo> for CardDriverActivityInfo {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardDriverActivityInfo> {
        let activity_info = reader.read_u16::<BigEndian>()?;
        Ok(CardDriverActivityInfo::new(ActivityCard::Card, activity_info))
    }
}

#[derive(Debug)]
pub struct CardDriverActivityRecord {
    pub record_date: TimeReal,
    pub daily_presence_counter: String,
    pub day_distance: u16,
    pub activity_infos: Vec<CardDriverActivityInfo>,
}

impl Readable<CardDriverActivityRecord> for CardDriverActivityRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardDriverActivityRecord> {
        let position = reader.pos()?;
        let reader_length = reader.len()?;
        let _activity_previous_record_length = reader.read_u16::<BigEndian>()?;
        let activity_record_length = reader.read_u16::<BigEndian>()?;
        if activity_record_length % 2 != 0 {
            return Err(Error::CardActivityRecord("Card Activity Record Length is not even".to_owned()));
        }
        let record_date = TimeReal::read(reader)?;
        let daily_presence_counter = BCDString::decode(&reader.read_into_vec(2)?);
        let day_distance = reader.read_u16::<BigEndian>()?;

        if activity_record_length == 0 {
            return Ok(Self { record_date, daily_presence_counter, day_distance, activity_infos: Vec::new() });
        }

        let mut end_pos = position + activity_record_length as usize;
        if end_pos >= reader_length {
            end_pos -= reader_length;
        }
        let mut activity_infos: Vec<CardDriverActivityInfo> = Vec::new();
        let mut activity_counter = 0;
        while reader.pos()? != end_pos {
            let activity_info = CardDriverActivityInfo::read(reader)?;
            activity_infos.push(activity_info);
            activity_counter += 1;
            if activity_counter > 2440 {
                return Err(Error::CardActivityRecord(
                    "Card with ActivityDailyRecord has more than 2440 activities in day".to_owned(),
                ));
            }
        }

        Ok(Self { record_date, daily_presence_counter, day_distance, activity_infos })
    }
}

#[derive(Debug)]
pub struct CardDriverActivity {
    pub daily_records: Vec<CardDriverActivityRecord>,
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

        let mut daily_records: Vec<CardDriverActivityRecord> = Vec::new();
        let mut activity_reader =
            BinRingMemoryBuffer::new_with_offset(activity_daily_records_raw, activity_pointer_oldest_day_record as usize);

        loop {
            let position = activity_reader.pos()?;
            let record = CardDriverActivityRecord::read(&mut activity_reader)?;
            daily_records.push(record);

            if position == activity_pointer_newest_record as usize {
                break;
            }
        }

        Ok(Self { daily_records })
    }
}
