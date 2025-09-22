use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use serde_json::json;

#[allow(dead_code)]
const FORMAT_UTC: &str = "%Y-%m-%d %H:%M:%S UTC";

#[allow(dead_code)]
pub fn from_obj_to_string<T: Serialize>(message: &T) -> String {
    let json_obj = json!(message);
    json_obj.to_string()
}

#[allow(dead_code)]
pub fn serialize_option_utc_date_time<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date.is_some() {
        true => {
            let s = format!("{}", date.as_ref().unwrap().format(FORMAT_UTC));
            serializer.serialize_str(&s)
        }
        false => serializer.serialize_none(),
    }
}
