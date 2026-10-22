use serde::Serialize;

use crate::Readable;
use crate::tacho::{TimeReal, VehicleRegistrationIdentification};

/// Information about the actual usage of the card (Annex 1C requirement
/// 273, 298, 326, and 349).
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardCurrentUse {
    #[cfg_attr(feature = "typescript", ts(rename = "sessionOpenTime"))]
    /// Zero means no current session; the raw timestamp is retained for writing.
    pub session_open_time: TimeReal,
    #[cfg_attr(feature = "typescript", ts(rename = "sessionOpenVehicle"))]
    pub session_open_vehicle: VehicleRegistrationIdentification,
}

impl Readable for CardCurrentUse {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardCurrentUse> {
        let session_open_time = TimeReal::read(reader)?;
        let session_open_vehicle = VehicleRegistrationIdentification::read(reader)?;
        Ok(Self { session_open_time, session_open_vehicle })
    }
}

impl Serialize for CardCurrentUse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("CardCurrentUse", 2)?;
        // Appendix 1, 2.16: card removal clears sessionOpenTime to zero.
        let session_open_time = (self.session_open_time.data != 0).then_some(&self.session_open_time);
        state.serialize_field("sessionOpenTime", &session_open_time)?;
        state.serialize_field("sessionOpenVehicle", &self.session_open_vehicle)?;
        state.end()
    }
}
