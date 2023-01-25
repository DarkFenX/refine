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
    util::{Error, Result},
};

use super::{data::CacheData, key::Key};

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

    fn load_cache(&mut self) -> Result<()> {
        let full_path = get_full_path(&self.folder, &self.name);
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(|e| Error::new(format!("unable to open cache for reading: {}", e)))?;
        let mut raw = Vec::new();
        zstd::stream::copy_decode(file, &mut raw)
            .map_err(|e| Error::new(format!("unable to decompress cache: {}", e)))?;
        let cache = serde_json::from_slice::<CacheData>(&raw)
            .map_err(|e| Error::new(format!("unable to decode cache: {}", e)))?;
        update_memory_cache(cache, self);
        Ok(())
    }

    fn update_cache(&mut self, data: ch::CHData, fingerprint: String) {
        // Update persistent cache
        let cache = CacheData::new(
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
        update_memory_cache(cache, self);
    }
}

fn get_full_path(folder: &PathBuf, name: &String) -> PathBuf {
    folder.join(format!("{}.json.zst", name))
}

fn write_cache(folder: &PathBuf, name: &String, cache: &CacheData) {
    let full_path = get_full_path(folder, name);
    let file = match OpenOptions::new().create(true).write(true).open(full_path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("unable to open cache file for writing: {}", e);
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

fn update_memory_cache(cache: CacheData, handler: &mut JsonFileCHandler) {
    move_vec_to_map(cache.items, &mut handler.storage_items);
    move_vec_to_map(cache.attrs, &mut handler.storage_attrs);
    move_vec_to_map(cache.effects, &mut handler.storage_effects);
    move_vec_to_map(cache.mutas, &mut handler.storage_mutas);
    move_vec_to_map(cache.buffs, &mut handler.storage_buffs);
    handler.fingerprint = cache.fingerprint;
}

fn move_vec_to_map<T>(vec: Vec<T>, map: &mut HashMap<ReeInt, T>)
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
