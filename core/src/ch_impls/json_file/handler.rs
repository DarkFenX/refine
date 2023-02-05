use std::{
    collections::HashMap,
    fmt,
    fs::{create_dir_all, OpenOptions},
    io,
    path::PathBuf,
    sync::Arc,
};

use log;

use crate::{
    ch,
    ct::{Attr, Buff, Effect, Item, Muta},
    IntError, IntResult, ReeInt,
};

use super::{data::CacheData, key::Key};

/// A struct for handling compressed JSON cache.
pub struct JsonFileCHandler {
    folder: PathBuf,
    name: String,
    storage_items: HashMap<ReeInt, Arc<Item>>,
    storage_attrs: HashMap<ReeInt, Arc<Attr>>,
    storage_effects: HashMap<ReeInt, Arc<Effect>>,
    storage_mutas: HashMap<ReeInt, Arc<Muta>>,
    storage_buffs: HashMap<ReeInt, Arc<Buff>>,
    fingerprint: String,
}
impl JsonFileCHandler {
    /// Constructs new `JsonFileCHandler` using full path to cache folder and file name (without
    /// extension).
    pub fn new<F: Into<PathBuf>, N: Into<String>>(folder: F, name: N) -> JsonFileCHandler {
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
    fn get_full_path(&self) -> PathBuf {
        self.folder.join(format!("{}.json.zst", self.name))
    }
    fn create_cache_folder(&self) -> IntResult<()> {
        match create_dir_all(&self.folder) {
            Ok(_) => Ok(()),
            Err(e) => {
                match e.kind() {
                    // It's fine if it already exists for our purposes
                    io::ErrorKind::AlreadyExists => Ok(()),
                    _ => Err(IntError::new(format!("unable to create cache folder: {}", e))),
                }
            }
        }
    }
    fn update_memory_cache(&mut self, cache: CacheData) {
        move_vec_to_map(cache.items, &mut self.storage_items);
        move_vec_to_map(cache.attrs, &mut self.storage_attrs);
        move_vec_to_map(cache.effects, &mut self.storage_effects);
        move_vec_to_map(cache.mutas, &mut self.storage_mutas);
        move_vec_to_map(cache.buffs, &mut self.storage_buffs);
        self.fingerprint = cache.fingerprint;
    }
    fn update_persistent_cache(&self, cache: &CacheData) {
        let full_path = self.get_full_path();
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
}
impl fmt::Debug for JsonFileCHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JsonFileCHandler(\"{}\")",
            self.get_full_path().to_str().unwrap_or("<error>")
        )
    }
}
impl ch::CacheHandler for JsonFileCHandler {
    /// Get cached item.
    fn get_item(&self, id: ReeInt) -> Option<Arc<Item>> {
        self.storage_items.get(&id).cloned()
    }
    /// Get cached attribute.
    fn get_attr(&self, id: ReeInt) -> Option<Arc<Attr>> {
        self.storage_attrs.get(&id).cloned()
    }
    /// Get cached effect.
    fn get_effect(&self, id: ReeInt) -> Option<Arc<Effect>> {
        self.storage_effects.get(&id).cloned()
    }
    /// Get cached mutaplasmid.
    fn get_muta(&self, id: ReeInt) -> Option<Arc<Muta>> {
        self.storage_mutas.get(&id).cloned()
    }
    /// Get cached warfare buff.
    fn get_buff(&self, id: ReeInt) -> Option<Arc<Buff>> {
        self.storage_buffs.get(&id).cloned()
    }
    /// Get cached data fingerprint.
    fn get_fingerprint(&self) -> &str {
        &self.fingerprint
    }
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> ch::Result<()> {
        let full_path = self.get_full_path();
        let file = OpenOptions::new()
            .read(true)
            .open(full_path)
            .map_err(|e| IntError::new(format!("unable to open cache for reading: {}", e)))?;
        let mut raw = Vec::new();
        zstd::stream::copy_decode(file, &mut raw)
            .map_err(|e| IntError::new(format!("unable to decompress cache: {}", e)))?;
        let cache = serde_json::from_slice::<CacheData>(&raw)
            .map_err(|e| IntError::new(format!("unable to deserealize cache: {}", e)))?;
        self.update_memory_cache(cache);
        Ok(())
    }
    /// Update data in handler with passed data.
    fn update_cache(&mut self, ch_data: ch::Data, fingerprint: String) {
        // Update persistent cache
        let cache = CacheData::new(
            ch_data.items,
            ch_data.attrs,
            ch_data.mutas,
            ch_data.effects,
            ch_data.buffs,
            fingerprint,
        );
        match self.create_cache_folder() {
            Ok(_) => self.update_persistent_cache(&cache),
            Err(e) => log::error!("unable to create cache folder: {}", e),
        }
        // Update memory cache
        self.update_memory_cache(cache);
    }
}

fn move_vec_to_map<T>(vec: Vec<T>, map: &mut HashMap<ReeInt, Arc<T>>)
where
    T: Key,
{
    map.clear();
    map.shrink_to_fit();
    map.reserve(vec.len());
    vec.into_iter().for_each(|v| {
        map.insert(v.get_key(), Arc::new(v));
    });
}
