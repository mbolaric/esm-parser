use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::{ActivitySource, ActivityType, CardSlotNumber, CardStatus, DrivingStatus};
use crate::{ReadableWithParams, Result};

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
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum ActivityCard {
    Vu,
    Card,
}

/// This data type enables to code, within a two bytes word, a slot status at
/// 00:00 and/or a driver status at 00:00 and/or changes of activity and/or
/// changes of driving status and/or changes of card status for a driver or a co-driver.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ActivityChangeInfo {
    /// The source of the activity, either from the VU or a card.
    #[serde(rename = "activityCard")]
    pub activity_card: ActivityCard,
    /// The raw 16-bit word containing the activity change information.
    #[serde(rename = "activityInfo")]
    pub activity_info: u16,
    /// Whether a valid driver or workshop card is inserted in the relevant slot.
    #[serde(rename = "cardStatus")]
    pub card_status: CardStatus,
    /// Bit 14: single/crew while the card is inserted, or unknown/known following activity otherwise.
    #[serde(rename = "drivingStatus")]
    pub driving_status: DrivingStatus,
    /// The meaningful slot for this change, or unknown when the encoded slot is not relevant.
    #[serde(rename = "cardSlot")]
    pub card_slot: CardSlotNumber,
    /// The literal slot encoded in bit 15, including records where that bit is not semantically relevant.
    #[serde(rename = "encodedCardSlot")]
    pub encoded_card_slot: CardSlotNumber,
    /// The source of the activity (automatic or manual).
    #[serde(rename = "activitySource")]
    pub activity_source: ActivitySource,
    /// The type of activity (e.g., driving, working, available, break).
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    /// The literal activity encoded in bits 12-11, including records where those bits are not semantically relevant.
    #[serde(rename = "encodedActivityType")]
    pub encoded_activity_type: ActivityType,
    /// Whether sequence context identifies this card record as the inserted-to-not-inserted withdrawal transition.
    #[serde(rename = "isCardWithdrawal")]
    pub is_card_withdrawal: bool,
    /// The time of the activity change in minutes since midnight.
    #[serde(rename = "timeInMin")]
    pub time_in_min: u16,
}

// Bit layout of the word, 'scpaattttttttttt'B (Annex IB / Annex IC, Appendix 1, ActivityChangeInfo).
const SLOT_BIT: u16 = 1 << 15;
const STATUS_BIT: u16 = 1 << 14;
const CARD_STATUS_SHIFT: u16 = 13;
const ACTIVITY_SHIFT: u16 = 11;
const TIME_MASK: u16 = 0x7FF;

impl ActivityChangeInfo {
    /// Creates a new `ActivityChangeInfo` by decoding the word `'scpaattttttttttt'B`:
    ///     - bit 15 `s`: card slot, 0 = driver, 1 = co-driver.
    ///     - bit 14 `c`: driving status, 0 = single, 1 = crew. In a card record whose card is not inserted (`p` = 1)
    ///         it is the following-activity status instead: 0 = unknown, 1 = known (manually entered).
    ///     - bit 13 `p`: card status, 0 = inserted, 1 = not inserted.
    ///     - bits 12-11 `aa`: activity; not relevant in a card record with `p` = 1 and `c` = 0.
    ///     - bits 10-0: time of the change in minutes since 00:00.
    pub fn new(activity_card: ActivityCard, activity_info: u16) -> Self {
        let time_in_min = activity_info & TIME_MASK;
        let card_status = (((activity_info >> CARD_STATUS_SHIFT) & 1) as u8).into();
        // The variants name both meanings of bit 14; read `driving_status` together with `card_status`.
        let driving_status =
            if activity_info & STATUS_BIT == 0 { DrivingStatus::SingleOrUnknown } else { DrivingStatus::CrewOrKnown };
        let encoded_card_slot = ActivityChangeInfo::decode_card_slot(activity_info);
        let card_slot_valid = ActivityChangeInfo::get_slot_valid(&activity_card, &card_status);
        let card_slot = ActivityChangeInfo::get_card_slot(card_slot_valid, activity_info);
        let activity_source = ActivityChangeInfo::get_activity_source(&activity_card, &card_status, &driving_status);
        let activity_type = ActivityChangeInfo::get_activity_type(&activity_source, activity_info);
        let encoded_activity_type = ActivityChangeInfo::decode_activity_type(activity_info);

        Self {
            activity_card,
            activity_info,
            card_status,
            driving_status,
            card_slot,
            encoded_card_slot,
            activity_source,
            activity_type,
            encoded_activity_type,
            is_card_withdrawal: false,
            time_in_min,
        }
    }

    /// A card record without the card inserted is manual when bit 14 says the following activity is known, and
    /// unknown otherwise; everything else was recorded by the equipment.
    fn get_activity_source(
        activity_card: &ActivityCard,
        card_status: &CardStatus,
        driving_status: &DrivingStatus,
    ) -> ActivitySource {
        match (activity_card, card_status, driving_status) {
            (ActivityCard::Card, CardStatus::NotInserted, DrivingStatus::CrewOrKnown) => ActivitySource::Manual,
            (ActivityCard::Card, CardStatus::NotInserted, _) => ActivitySource::Unknown,
            _ => ActivitySource::Automatic,
        }
    }

    /// The activity bits carry no meaning when the source is unknown.
    fn get_activity_type(activity_source: &ActivitySource, activity_info: u16) -> ActivityType {
        if *activity_source == ActivitySource::Unknown {
            return ActivityType::Unknown;
        }
        ActivityChangeInfo::decode_activity_type(activity_info)
    }

    fn decode_activity_type(activity_info: u16) -> ActivityType {
        ((activity_info >> ACTIVITY_SHIFT) & 0b11).into()
    }

    /// Determines the card slot number.
    fn get_card_slot(card_slot_valid: bool, activity_info: u16) -> CardSlotNumber {
        if !card_slot_valid {
            return CardSlotNumber::Unknown;
        }
        ActivityChangeInfo::decode_card_slot(activity_info)
    }

    fn decode_card_slot(activity_info: u16) -> CardSlotNumber {
        if activity_info & SLOT_BIT == 0 { CardSlotNumber::Driver } else { CardSlotNumber::CoDriver }
    }

    /// Checks if the card slot information is valid.
    fn get_slot_valid(activity_card: &ActivityCard, card_status: &CardStatus) -> bool {
        if *activity_card == ActivityCard::Vu {
            return true;
        }
        *card_status == CardStatus::Inserted
    }

    /// Marks inserted-to-not-inserted transitions after the complete card-day sequence is available.
    pub(crate) fn identify_card_withdrawals(
        changes: &mut [ActivityChangeInfo],
        initial_card_inserted: Option<bool>,
    ) -> Option<bool> {
        let mut previous_card_inserted = initial_card_inserted;

        for change in changes.iter_mut() {
            let card_inserted = change.card_status == CardStatus::Inserted;
            if change.activity_card == ActivityCard::Card
                && previous_card_inserted == Some(true)
                && change.card_status == CardStatus::NotInserted
            {
                change.is_card_withdrawal = true;
                change.card_slot = ActivityChangeInfo::decode_card_slot(change.activity_info);
            }
            previous_card_inserted = Some(card_inserted);
        }

        changes.last().map(|change| change.card_status == CardStatus::Inserted)
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

#[cfg(test)]
mod tests {
    use super::*;

    const REST: u16 = 0b00;
    const AVAILABILITY: u16 = 0b01;
    const WORK: u16 = 0b10;
    const DRIVING: u16 = 0b11;

    // Builds the word 'scpaattttttttttt'B from its named bits.
    fn word(slot: u16, status: u16, card_status: u16, activity: u16, minutes: u16) -> u16 {
        (slot << 15) | (status << 14) | (card_status << 13) | (activity << 11) | minutes
    }

    fn decoded(activity_card: ActivityCard, activity_info: u16) -> ActivityChangeInfo {
        ActivityChangeInfo::new(activity_card, activity_info)
    }

    #[test]
    fn decodes_an_inserted_card_driving_single_in_the_driver_slot() {
        let info = decoded(ActivityCard::Card, word(0, 0, 0, DRIVING, 480));
        assert_eq!(info.card_slot, CardSlotNumber::Driver);
        assert_eq!(info.driving_status, DrivingStatus::SingleOrUnknown);
        assert_eq!(info.card_status, CardStatus::Inserted);
        assert_eq!(info.activity_source, ActivitySource::Automatic);
        assert_eq!(info.activity_type, ActivityType::Driving);
        assert_eq!(info.time_in_min, 480);
    }

    #[test]
    fn reads_the_driving_status_from_bit_14_not_from_the_slot_bit() {
        let co_driver_single = decoded(ActivityCard::Card, word(1, 0, 0, AVAILABILITY, 600));
        assert_eq!(co_driver_single.card_slot, CardSlotNumber::CoDriver);
        assert_eq!(co_driver_single.driving_status, DrivingStatus::SingleOrUnknown);

        let driver_crew = decoded(ActivityCard::Card, word(0, 1, 0, DRIVING, 600));
        assert_eq!(driver_crew.card_slot, CardSlotNumber::Driver);
        assert_eq!(driver_crew.driving_status, DrivingStatus::CrewOrKnown);
    }

    #[test]
    fn decodes_an_unknown_following_activity_when_the_card_is_not_inserted() {
        let info = decoded(ActivityCard::Card, word(0, 0, 1, REST, 1200));
        assert_eq!(info.card_status, CardStatus::NotInserted);
        assert_eq!(info.card_slot, CardSlotNumber::Unknown);
        assert_eq!(info.encoded_card_slot, CardSlotNumber::Driver);
        assert_eq!(info.activity_source, ActivitySource::Unknown);
        assert_eq!(info.activity_type, ActivityType::Unknown);
        assert_eq!(info.encoded_activity_type, ActivityType::Rest);
        assert!(!info.is_card_withdrawal);
    }

    #[test]
    fn keeps_a_known_manually_entered_following_activity() {
        let info = decoded(ActivityCard::Card, word(0, 1, 1, WORK, 1200));
        assert_eq!(info.driving_status, DrivingStatus::CrewOrKnown);
        assert_eq!(info.activity_source, ActivitySource::Manual);
        assert_eq!(info.activity_type, ActivityType::Work);
        assert!(!info.is_card_withdrawal);
    }

    #[test]
    fn preserves_the_slot_and_encoded_activity_of_a_card_withdrawal() {
        let mut changes = vec![
            decoded(ActivityCard::Card, word(1, 0, 0, WORK, 480)),
            decoded(ActivityCard::Card, word(1, 0, 1, WORK, 720)),
            decoded(ActivityCard::Card, word(0, 0, 1, REST, 780)),
        ];

        ActivityChangeInfo::identify_card_withdrawals(&mut changes, None);

        let withdrawal = &changes[1];
        assert!(withdrawal.is_card_withdrawal);
        assert_eq!(withdrawal.card_slot, CardSlotNumber::CoDriver);
        assert_eq!(withdrawal.encoded_activity_type, ActivityType::Work);
        assert_eq!(withdrawal.activity_type, ActivityType::Unknown);

        let following_unknown_activity = &changes[2];
        assert!(!following_unknown_activity.is_card_withdrawal);
        assert_eq!(following_unknown_activity.card_slot, CardSlotNumber::Unknown);
    }

    #[test]
    fn decodes_a_vehicle_unit_slot_without_a_card_as_recorded_by_the_equipment() {
        let info = decoded(ActivityCard::Vu, word(1, 0, 1, DRIVING, 30));
        assert_eq!(info.card_status, CardStatus::NotInserted);
        assert_eq!(info.card_slot, CardSlotNumber::CoDriver);
        assert_eq!(info.driving_status, DrivingStatus::SingleOrUnknown);
        assert_eq!(info.activity_source, ActivitySource::Automatic);
        assert_eq!(info.activity_type, ActivityType::Driving);
    }

    #[test]
    fn decodes_the_last_minute_of_the_day() {
        assert_eq!(decoded(ActivityCard::Card, word(0, 0, 0, REST, 1439)).time_in_min, 1439);
    }

    #[test]
    fn preserves_an_out_of_range_minute_for_caller_validation() {
        let info = ActivityChangeInfo::new(ActivityCard::Card, word(0, 0, 0, REST, 1440));
        assert_eq!(info.time_in_min, 1440);
    }
}
