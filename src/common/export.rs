use serde::Serialize;

use crate::Result;

pub trait Export {
    fn to_json(&self) -> Result<String>
    where
        Self: Serialize,
    {
        Ok(serde_json::to_string(self)?)
    }
}
