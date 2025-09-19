use serde::Serialize;

use crate::{Readable, tacho::TimeReal};

/// Information, stored in a vehicle unit, related to over speeding events
/// since the last over speeding control (Annex 1B requirement 095 and
/// Annex 1C requirement 117).
#[derive(Debug, Serialize)]
pub struct VuOverSpeedingControlData {
    #[serde(rename = "lastOverspeedControlTime")]
    pub last_overspeed_control_time: TimeReal,
    #[serde(rename = "firstOverspeedSince")]
    pub first_overspeed_since: TimeReal,
    #[serde(rename = "numberOfOverspeedSince")]
    pub number_of_overspeed_since: u8,
}

impl Readable<VuOverSpeedingControlData> for VuOverSpeedingControlData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuOverSpeedingControlData> {
        let last_overspeed_control_time = TimeReal::read(reader)?;
        let first_overspeed_since = TimeReal::read(reader)?;
        let number_of_overspeed_since = reader.read_u8()?;
        Ok(Self { last_overspeed_control_time, first_overspeed_since, number_of_overspeed_since })
    }
}
