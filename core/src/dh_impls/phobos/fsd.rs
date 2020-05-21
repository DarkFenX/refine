use std::collections::HashMap;

use log;

use crate::defines::ReeInt;
use crate::dh;

use super::error::{Error, Result};

#[derive(Debug)]
struct Flattened {
    data: HashMap<ReeInt, serde_json::Value>,
    failed: u32,
}

pub(super) fn handle<T, U>(unprocessed: serde_json::Value, key_name: &'static str) -> dh::Result<dh::Container<U>>
where
    T: serde::de::DeserializeOwned + Into<U>,
{
    let flattened = flatten(unprocessed, key_name)?;
    convert::<T, U>(flattened, key_name)
}

fn flatten(json: serde_json::Value, key_name: &'static str) -> Result<Flattened> {
    match json {
        serde_json::Value::Object(outer_map) => {
            let mut data = HashMap::new();
            let mut failed: u32 = 0;
            for (k, v) in outer_map.into_iter() {
                match (v, k.parse::<ReeInt>()) {
                    (serde_json::Value::Object(mut inner_map), Ok(id)) => {
                        inner_map.insert(key_name.to_owned(), serde_json::Value::Number(id.into()));
                        data.insert(id, serde_json::Value::Object(inner_map));
                    }
                    (_, Err(e)) => {
                        log::debug!("error while flattening due to ID parsing: {}", e);
                        failed += 1;
                    }
                    (_, Ok(id)) => {
                        log::debug!("error while flattening due to non-map item with ID {}", id);
                        failed += 1;
                    }
                }
            }
            Ok(Flattened { data, failed })
        }
        _ => Err(Error::new(
            "FSD Lite decomposition failed: highest-level structure is not a map",
        )),
    }
}

fn convert<T, U>(flattened: Flattened, key_name: &'static str) -> dh::Result<dh::Container<U>>
where
    T: serde::de::DeserializeOwned + Into<U>,
{
    let mut data = Vec::new();
    let mut failed: u32 = flattened.failed;
    for (id, json) in flattened.data.into_iter() {
        match serde_json::from_value::<T>(json) {
            Ok(r) => data.push(r.into()),
            Err(e) => {
                log::debug!("deserialization error for {}={}: {}", key_name, id, e);
                failed += 1;
            }
        }
    }
    Ok(dh::Container::new(data, failed))
}
