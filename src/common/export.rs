use serde::Serialize;

use crate::Result;

/// Export data to JSON or XML.
pub trait Export {
    /// Serializes the `TachographData` to a JSON string.
    ///
    /// # Arguments
    ///
    /// * `data` - The `TachographData` to serialize.
    ///
    /// # Returns
    ///
    /// A `Result` containing the JSON string or an `Error` if serialization fails.
    fn to_json(&self) -> Result<String>
    where
        Self: Serialize,
    {
        Ok(serde_json::to_string(self)?)
    }

    /// Serializes the `TachographData` to an XML string.
    ///
    /// # Arguments
    ///
    /// * `data` - The `TachographData` to serialize.
    ///
    /// # Returns
    ///
    /// A `Result` containing the XML string or an `Error` if serialization fails.
    fn to_xml(&self) -> Result<String>
    where
        Self: Serialize,
    {
        Ok(quick_xml::se::to_string(self)?)
    }
}
