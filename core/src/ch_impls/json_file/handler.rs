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

use super::key::Key;

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
    // fn get_item(&self, id: ReeInt) -> Result<Item> {
    // }
    // fn get_attr(&self, id: ReeInt) -> Result<Attr> {
    // }
    // fn get_effect(&self, id: ReeInt) -> Result<Effect> {
    // }
    // fn get_muta(&self, id: ReeInt) -> Result<Muta> {
    // }
    // fn get_buff(&self, id: ReeInt) -> Result<Buff> {
    // }
    // fn get_fingerprint(&self) -> Result<String> {
    // }
    fn update_cache(&mut self, data: ch::Container, fingerprint: String) {
        move_data(data.items, &mut self.storage_items);
        move_data(data.attrs, &mut self.storage_attrs);
        move_data(data.effects, &mut self.storage_effects);
        move_data(data.mutas, &mut self.storage_mutas);
        move_data(data.buffs, &mut self.storage_buffs);
        self.fingerprint = Some(fingerprint);
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
