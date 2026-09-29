use binary_data::{BinSeek, ReadBytes};
use log::debug;

use super::{TachographHeader, VUFilesList, VUTransferResponseParameterID};
use crate::VU_HEADER_MAGIC_NUMBER;
use crate::error::Result;
use crate::tacho::{VUTransferResponseParameter, VUTransferResponseParameterItem};

pub trait VUData<D> {
    fn get_header(&self) -> &TachographHeader;
    fn get_data(&self) -> &[VUTransferResponseParameterItem<D>];
}

pub(crate) struct VuReader;

impl VuReader {
    pub(crate) fn from_data_with_files<R, D, F>(
        reader: &mut R,
        parse_trep: F,
    ) -> Result<(Vec<VUTransferResponseParameterItem<D>>, VUFilesList)>
    where
        R: ReadBytes + BinSeek,
        D: VUTransferResponseParameter,
        F: Fn(VUTransferResponseParameterID, &mut R) -> Result<D>,
    {
        let mut position: u32 = 0;
        let mut data_position: usize = 0;
        let mut transfer_res_params: Vec<VUTransferResponseParameterItem<D>> = Vec::new();
        let mut data_files: VUFilesList = Vec::new();

        while reader.pos()? < reader.len()? {
            position += 1;
            let mut vu_trep: VUTransferResponseParameterID = VUTransferResponseParameterID::Unknown;
            let mut magic_number = reader.read_u8()?;
            if magic_number == VU_HEADER_MAGIC_NUMBER {
                vu_trep = VUTransferResponseParameterID::from(reader.read_u8()?);
                data_position = reader.pos()?;
            } else {
                while magic_number != VU_HEADER_MAGIC_NUMBER && reader.pos()? < reader.len()? - 1 {
                    magic_number = reader.read_u8()?;
                    if magic_number == VU_HEADER_MAGIC_NUMBER {
                        vu_trep = VUTransferResponseParameterID::from(reader.read_u8()?);
                        if !vu_trep.is_unknown() {
                            data_position = reader.pos()?;
                            break;
                        }
                    }
                }
            }

            if !vu_trep.is_unknown() {
                debug!("VuReader::from_data - Trep ID: {:?} on position: {}", vu_trep, data_position);
                let start_pos = reader.pos()?;
                let data = parse_trep(vu_trep, reader)?;
                let end_pos = reader.pos()?;

                let trep_len = end_pos.saturating_sub(start_pos);
                reader.seek(start_pos)?;
                let raw_bytes = reader.read_into_vec(trep_len as u32)?;
                reader.seek(end_pos)?;

                if let Some(file_data) = data.extract_file_data(vu_trep, position, &raw_bytes) {
                    data_files.push(file_data);
                }

                let is_oddball_crash_dump = data.is_oddball_crash_dump();
                transfer_res_params.push(VUTransferResponseParameterItem::<D> { type_id: vu_trep, position, data });

                if is_oddball_crash_dump {
                    return Ok((transfer_res_params, data_files));
                }
            }
        }
        Ok((transfer_res_params, data_files))
    }
}
