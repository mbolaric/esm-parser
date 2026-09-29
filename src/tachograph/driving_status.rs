use serde::Serialize;

/// Bit 14 (`c`) of an `ActivityChangeInfo` word. While a card is inserted, and in every vehicle unit record, it is the
/// driving status (single or crew); in a driver card record whose card is not inserted it is the following-activity
/// status (unknown or known, i.e. manually entered). Read it together with the record's card status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[repr(u8)]
pub enum DrivingStatus {
    /// The driving status is unknown.
    Unknown = 255,
    /// Single driver (card inserted), or unknown following activity (card not inserted).
    SingleOrUnknown = 0,
    /// Crew, two valid driver cards inserted, or manually entered following activity (card not inserted).
    CrewOrKnown = 1,
}
