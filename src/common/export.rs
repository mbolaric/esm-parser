use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use serde::Serialize;
use std::io::Cursor;

use crate::Result;

/// Export data to JSON or XML.
pub trait Export {
    /// Serializes the data structure to a JSON string.
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

    /// Serializes the data structure to a 'pretty' JSON string.
    ///
    /// # Returns
    ///
    /// A `Result` containing the 'pretty' JSON string or an `Error` if serialization fails.
    fn to_json_pretty(&self) -> Result<String>
    where
        Self: Serialize,
    {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Serializes the data structure to an XML string.
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

    /// Serializes the data structure to an 'pretty' XML string.
    ///
    /// # Returns
    ///
    /// A `Result` containing the 'pretty' XML string or an `Error` if serialization fails.
    fn to_xml_pretty(&self) -> Result<String>
    where
        Self: Serialize,
    {
        let ugly_xml = quick_xml::se::to_string(self)?;
        let pretty_xml = pretty_print_xml(&ugly_xml)?;
        Ok(pretty_xml)
    }
}

/// Formats an XML string with indentation.
///
/// This function takes a string containing XML data, parses it event by event,
/// and then rewrites it with indentation to make it more human-readable.
/// Each level of nesting is indented with four spaces.
///
/// # Arguments
///
/// * `xml` - A string slice that holds the XML data to be formatted.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(String)`: A new string containing the formatted, indented XML.
/// - `Err(quick_xml::Error)`: An error if the input string is not well-formed XML
///   or if any other parsing or writing error occurs.
fn pretty_print_xml(xml: &str) -> Result<String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);

    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(event) => writer.write_event(event)?,
            Err(e) => return Err(e.into()),
        }
    }

    let result = writer.into_inner().into_inner();
    // The '?' operator will automatically convert the FromUtf8Error into a quick_xml::Error
    Ok(String::from_utf8(result)?)
}
