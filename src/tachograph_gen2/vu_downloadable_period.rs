use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::TimeReal;
use crate::{Readable, Result};

/// Oldest and latest dates for which a vehicle unit holds data related to
/// drivers activities (Annex 1B requirements 081, 084 or 087 and
/// Annex 1C requirements 102, 105, 108).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VuDownloadablePeriod"))]
pub struct VuDownloadablePeriod {
    #[serde(rename = "minDownloadableTime")]
    pub min_downloadable_time: TimeReal,
    #[serde(rename = "maxDownloadableTime")]
    pub max_downloadable_time: TimeReal,
}

impl Readable<VuDownloadablePeriod> for VuDownloadablePeriod {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuDownloadablePeriod> {
        let min_downloadable_time = TimeReal::read(reader)?;
        let max_downloadable_time = TimeReal::read(reader)?;
        Ok(Self { min_downloadable_time, max_downloadable_time })
    }
}
