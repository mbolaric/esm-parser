use crate::{
    Error, Result,
    tacho::{CardFilesMap, VerifyResult},
};

pub fn verify(_data_files: &CardFilesMap, _erca_pk: &[u8]) -> Result<VerifyResult> {
    Err(Error::NotImplemented)
}
