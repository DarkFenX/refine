use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use crate::{
    ch,
    ct::{Attr, Buff, Effect, Item, Muta},
    defines::ReeInt,
    util::Result,
};

use super::{container::Container, key::Key};

/// A struct for handling compressed JSON cache
pub struct JsonFileCHandler {
    cache_path: PathBuf,
    storage_items: HashMap<ReeInt, Item>,
    storage_attrs: HashMap<ReeInt, Attr>,
    storage_effects: HashMap<ReeInt, Effect>,
    storage_mutas: HashMap<ReeInt, Muta>,
    storage_buffs: HashMap<ReeInt, Buff>,
    fingerprint: Option<String>,
}
impl JsonFileCHandler {
    /// Constructs new `JsonFileCHandler` using cache file path (path ending with .json.bz2).
    pub fn new<T: Into<PathBuf>>(path: T) -> JsonFileCHandler {
        JsonFileCHandler {
            cache_path: path.into(),
            storage_items: HashMap::new(),
            storage_attrs: HashMap::new(),
            storage_effects: HashMap::new(),
            storage_mutas: HashMap::new(),
            storage_buffs: HashMap::new(),
            fingerprint: None,
        }
    }
}
impl fmt::Debug for JsonFileCHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JsonFileCHandler(\"{}\")",
            self.cache_path.to_str().unwrap_or("<error>")
        )
    }
}
impl ch::CacheHandler for JsonFileCHandler {
    fn get_item(&self, id: ReeInt) -> Option<&Item> {
        self.storage_items.get(&id)
    }
    fn get_attr(&self, id: ReeInt) -> Option<&Attr> {
        self.storage_attrs.get(&id)
    }
    fn get_effect(&self, id: ReeInt) -> Option<&Effect> {
        self.storage_effects.get(&id)
    }
    fn get_muta(&self, id: ReeInt) -> Option<&Muta> {
        self.storage_mutas.get(&id)
    }
    fn get_buff(&self, id: ReeInt) -> Option<&Buff> {
        self.storage_buffs.get(&id)
    }
    fn get_fingerprint(&self) -> Option<&String> {
        self.fingerprint.as_ref()
    }
    fn update_cache(&mut self, data: ch::Container, fingerprint: String) {
        // Update memory cache
        move_data(data.items, &mut self.storage_items);
        move_data(data.attrs, &mut self.storage_attrs);
        move_data(data.effects, &mut self.storage_effects);
        move_data(data.mutas, &mut self.storage_mutas);
        move_data(data.buffs, &mut self.storage_buffs);
        self.fingerprint = Some(fingerprint);
        // Update persistent cache
        let data = Container::new(
            self.storage_items.values().collect(),
            self.storage_attrs.values().collect(),
            self.storage_mutas.values().collect(),
            self.storage_effects.values().collect(),
            self.storage_buffs.values().collect(),
            self.fingerprint.as_ref().unwrap(),
        );
        // let json = serde_json::json!(data).to_string();
    }
}

fn move_data<T>(vec: Vec<T>, map: &mut HashMap<ReeInt, T>)
where
    T: Key,
{
    map.clear();
    map.shrink_to_fit();
    map.reserve(vec.len());
    vec.into_iter().for_each(|v| {
        map.insert(v.get_key(), v);
    });
}
