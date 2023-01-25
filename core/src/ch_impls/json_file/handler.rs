use std::{
    collections::HashMap,
    fmt,
    fs::{create_dir_all, OpenOptions},
    io,
    path::PathBuf,
};

use log;

use crate::{
    ch,
    ct::{Attr, Buff, Effect, Item, Muta},
    defines::ReeInt,
};

use super::{container::Container, key::Key};

/// A struct for handling compressed JSON cache
pub struct JsonFileCHandler {
    folder: PathBuf,
    name: String,
    storage_items: HashMap<ReeInt, Item>,
    storage_attrs: HashMap<ReeInt, Attr>,
    storage_effects: HashMap<ReeInt, Effect>,
    storage_mutas: HashMap<ReeInt, Muta>,
    storage_buffs: HashMap<ReeInt, Buff>,
    fingerprint: String,
}
impl JsonFileCHandler {
    /// Constructs new `JsonFileCHandler` using cache file path (path ending with .json.bz2).
    pub fn new<T: Into<PathBuf>, U: Into<String>>(folder: T, name: U) -> JsonFileCHandler {
        JsonFileCHandler {
            folder: folder.into(),
            name: name.into(),
            storage_items: HashMap::new(),
            storage_attrs: HashMap::new(),
            storage_effects: HashMap::new(),
            storage_mutas: HashMap::new(),
            storage_buffs: HashMap::new(),
            fingerprint: String::new(),
        }
    }
}
impl fmt::Debug for JsonFileCHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JsonFileCHandler(\"{}\")",
            get_full_path(&self.folder, &self.name).to_str().unwrap_or("<error>")
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
    fn get_fingerprint(&self) -> &String {
        &self.fingerprint
    }
    fn update_cache(&mut self, data: ch::Container, fingerprint: String) {
        // Update persistent cache
        let cache = Container::new(
            data.items,
            data.attrs,
            data.mutas,
            data.effects,
            data.buffs,
            fingerprint,
        );
        match create_dir_all(&self.folder) {
            Ok(_) => write_cache(&self.folder, &self.name, &cache),
            Err(e) => {
                match e.kind() {
                    // We don't really care if it already exists, so just write the cache
                    io::ErrorKind::AlreadyExists => write_cache(&self.folder, &self.name, &cache),
                    _ => log::error!("unable to create cache folder: {}", e),
                };
            }
        }
        // Update memory cache
        move_data(cache.items, &mut self.storage_items);
        move_data(cache.attrs, &mut self.storage_attrs);
        move_data(cache.effects, &mut self.storage_effects);
        move_data(cache.mutas, &mut self.storage_mutas);
        move_data(cache.buffs, &mut self.storage_buffs);
        self.fingerprint = cache.fingerprint;
    }
}

fn get_full_path(folder: &PathBuf, name: &String) -> PathBuf {
    folder.join(format!("{}.json.zst", name))
}

fn write_cache(folder: &PathBuf, name: &String, cache: &Container) {
    let full_path = get_full_path(folder, name);
    let file = match OpenOptions::new().create(true).write(true).open(full_path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("unable to open cache file: {}", e);
            return;
        }
    };
    let json = serde_json::json!(&cache).to_string();
    match zstd::stream::copy_encode(json.as_bytes(), file, 7) {
        Ok(_) => (),
        Err(e) => {
            log::error!("unable to write cache file: {}", e);
            return;
        }
    };
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
