use binary_data::{BinSeek, ReadBytes};
use log::debug;

use super::{TachographHeader, VUTransferResponseParameterID};
use crate::{
    VU_HEADER_MAGIC_NUMBER,
    error::Result,
    tacho::{VUTransferResponseParameter, VUTransferResponseParameterItem},
};

pub trait VUData<D> {
    fn get_header(&self) -> &TachographHeader;
    fn get_data(&self) -> &Vec<VUTransferResponseParameterItem<D>>;
}

impl<D: VUTransferResponseParameter> dyn VUData<D> {
    pub fn from_data<R: ReadBytes + BinSeek>(
        reader: &mut R,
        parse_trep: &dyn Fn(VUTransferResponseParameterID, &mut R) -> Result<D>,
    ) -> Result<Vec<VUTransferResponseParameterItem<D>>> {
        let mut position: u32 = 0;
        let mut data_position: usize = 0;
        let mut transfer_res_params: Vec<VUTransferResponseParameterItem<D>> = Vec::new();

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
                            break;
                        }
                    }
                }
            }

            if !vu_trep.is_unknown() {
                debug!("VUData::from_data - Trep ID: {:?} on position: {}", vu_trep, data_position);
                let data = parse_trep(vu_trep.clone(), reader)?;
                let is_oddball_crash_dump = data.is_oddball_crash_dump();
                transfer_res_params.push(VUTransferResponseParameterItem::<D> { type_id: vu_trep.clone(), position, data });

                if is_oddball_crash_dump {
                    return Ok(transfer_res_params);
                }
            }
        }
        Ok(transfer_res_params)
    }
}
