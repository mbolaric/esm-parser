use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    ReadableWithParams, Result,
    tacho::{ActivitySource, ActivityType, CardSlotNumber, CardStatus, DrivingStatus},
};

#[derive(Debug)]
pub struct ActivityChangeInfoParams {
    pub activity_card: ActivityCard,
}

impl ActivityChangeInfoParams {
    pub fn new(activity_card: ActivityCard) -> Self {
        Self { activity_card }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ActivityCard {
    Vu,
    Card,
}

/// This data type enables to code, within a two bytes word, a slot status at
/// 00:00 and/or a driver status at 00:00 and/or changes of activity and/or
/// changes of driving status and/or changes of card status for a driver or a co-driver.
#[derive(Debug, Serialize)]
pub struct ActivityChangeInfo {
    /// The source of the activity, either from the VU or a card.
    #[serde(rename = "activityCard")]
    pub activity_card: ActivityCard,
    /// The raw 16-bit word containing the activity change information.
    #[serde(rename = "activityInfo")]
    pub activity_info: u16,
    /// The status of the card (inserted or removed).
    #[serde(rename = "cardStatus")]
    pub card_status: CardStatus,
    /// The driving status (single driver or crew).
    #[serde(rename = "drivingStatus")]
    pub driving_status: DrivingStatus,
    /// The card slot number (driver or co-driver).
    #[serde(rename = "cardSlot")]
    pub card_slot: CardSlotNumber,
    /// The source of the activity (automatic or manual).
    #[serde(rename = "activitySource")]
    pub activity_source: ActivitySource,
    /// The type of activity (e.g., driving, working, available, break).
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// The time of the activity change in minutes since midnight.
    #[serde(rename = "timeInMin")]
    pub time_in_min: u16,
}

impl ActivityChangeInfo {
    /// Creates a new `ActivityChangeInfo` instance by parsing a 16-bit integer.
    ///
    /// The `activity_info` is a bitfield with the following structure:
    /// - Bits 0-10: Time of the activity change in minutes since midnight (0-1439).
    /// - Bits 11-12: Activity type.
    /// - Bit 13: Card status (0 = inserted, 1 = removed).
    /// - Bit 14: Not used.
    /// - Bit 15: Driving status/card slot (0 = driver/single, 1 = co-driver/crew).
    pub fn new(activity_card: ActivityCard, activity_info: u16) -> Self {
        let card_status = (((activity_info >> 13) & 1) as u8).into();
        let driving_status = ActivityChangeInfo::get_driving_crew(&card_status, activity_info);
        let card_slot_valid = ActivityChangeInfo::get_slot_valid(&activity_card, &card_status);
        let card_slot = ActivityChangeInfo::get_card_slot(card_slot_valid, activity_info);
        let activity_source = ActivityChangeInfo::get_activity_source(&card_status, &driving_status);
        let activity_type = ActivityChangeInfo::get_activity_type(&activity_card, &activity_source, activity_info);
        let time_in_min = activity_info & 0x7FF;

        Self { activity_card, activity_info, card_status, driving_status, card_slot, activity_source, activity_type, time_in_min }
    }

    /// Determines the source of the activity based on card and driving status.
    fn get_activity_source(card_status: &CardStatus, driving_status: &DrivingStatus) -> ActivitySource {
        if *card_status == CardStatus::Removed {
            if *driving_status != DrivingStatus::SingleOrUnknown {
                return ActivitySource::Unknown;
            }
            return ActivitySource::Manual;
        }
        ActivitySource::Automatic
    }

    /// Determines the type of activity.
    fn get_activity_type(activity_card: &ActivityCard, activity_source: &ActivitySource, activity_info: u16) -> ActivityType {
        if *activity_source == ActivitySource::Unknown && *activity_card == ActivityCard::Card {
            return ActivityType::Unknown;
        }
        ((activity_info >> 11) & 0b11).into()
    }

    /// Determines the card slot number.
    fn get_card_slot(card_slot_valid: bool, activity_info: u16) -> CardSlotNumber {
        if !card_slot_valid {
            return CardSlotNumber::Unknown;
        }
        if (activity_info & (1 << 15)) == 0 {
            return CardSlotNumber::Driver;
        }
        CardSlotNumber::CoDriver
    }

    /// Checks if the card slot information is valid.
    fn get_slot_valid(activity_card: &ActivityCard, card_status: &CardStatus) -> bool {
        if *activity_card == ActivityCard::Vu {
            return true;
        }
        *card_status == CardStatus::Inserted
    }

    /// Determines the driving status (single driver or crew).
    fn get_driving_crew(card_status: &CardStatus, activity_info: u16) -> DrivingStatus {
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

impl ReadableWithParams<ActivityChangeInfo> for ActivityChangeInfo {
    type P = ActivityChangeInfoParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<ActivityChangeInfo> {
        let activity_card = params.activity_card.clone();
        let activity_info = reader.read_u16::<BigEndian>()?;
        Ok(ActivityChangeInfo::new(activity_card, activity_info))
    }
}
