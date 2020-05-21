use crate::defines::ReeInt;
use crate::dh;

use super::error::{Error, Result};

pub(super) fn handle<T, U>(unprocessed: serde_json::Value, key_name: &'static str) -> dh::Result<dh::Container<U>>
where
    T: serde::de::DeserializeOwned + Into<U>,
{
    let flattened = flatten(unprocessed, key_name)?;
    convert::<T, U>(flattened)
}

fn flatten(json: serde_json::Value, key_name: &'static str) -> Result<dh::Container<serde_json::Value>> {
    match json {
        serde_json::Value::Object(outer_map) => {
            let mut data = Vec::new();
            let mut failed: u32 = 0;
            for (k, v) in outer_map.into_iter() {
                match (v, k.parse::<ReeInt>()) {
                    (serde_json::Value::Object(mut inner_map), Ok(id)) => {
                        inner_map.insert(key_name.to_owned(), serde_json::Value::Number(id.into()));
                        data.push(serde_json::Value::Object(inner_map));
                    }
                    _ => failed += 1,
                }
            }
            Ok(dh::Container::new(data, failed))
        }
        _ => Err(Error::new(
            "FSD Lite decomposition failed: highest-level structure is not a map",
        )),
    }
}

fn convert<T, U>(flattened: dh::Container<serde_json::Value>) -> dh::Result<dh::Container<U>>
where
    T: serde::de::DeserializeOwned + Into<U>,
{
    let mut data = Vec::new();
    let mut failed: u32 = flattened.failed;
    for json in flattened.data {
        match serde_json::from_value::<T>(json) {
            Ok(r) => data.push(r.into()),
            _ => failed += 1,
        }
    }
    Ok(dh::Container::new(data, failed))
}
