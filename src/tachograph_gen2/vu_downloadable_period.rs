use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, tacho::TimeReal};

#[derive(Debug, Serialize)]
pub struct VuDownloadablePeriod {
    pub min_downloadable_time: TimeReal,
    pub max_downloadable_time: TimeReal,
}

impl Readable<VuDownloadablePeriod> for VuDownloadablePeriod {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuDownloadablePeriod> {
        let min_downloadable_time = TimeReal::read(reader)?;
        let max_downloadable_time = TimeReal::read(reader)?;
        Ok(Self { min_downloadable_time, max_downloadable_time })
    }
}
