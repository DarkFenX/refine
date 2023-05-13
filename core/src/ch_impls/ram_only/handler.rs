use std::{collections::HashMap, fmt, sync::Arc};

use crate::{
    ch,
    ct::{Attr, Buff, Effect, Item, Muta},
    defs::ReeInt,
    util::IntError,
};

use super::super::common::move_vec_to_map;

/// A struct for handling RAM-only cache.
pub struct RamOnlyCHandler {
    storage_items: HashMap<ReeInt, Arc<Item>>,
    storage_attrs: HashMap<ReeInt, Arc<Attr>>,
    storage_effects: HashMap<ReeInt, Arc<Effect>>,
    storage_mutas: HashMap<ReeInt, Arc<Muta>>,
    storage_buffs: HashMap<ReeInt, Arc<Buff>>,
}
impl RamOnlyCHandler {
    pub fn new() -> Self {
        Self {
            storage_items: HashMap::new(),
            storage_attrs: HashMap::new(),
            storage_effects: HashMap::new(),
            storage_mutas: HashMap::new(),
            storage_buffs: HashMap::new(),
        }
    }
}
impl fmt::Debug for RamOnlyCHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RamOnlyCHandler()")
    }
}
impl ch::CacheHandler for RamOnlyCHandler {
    /// Get cached item.
    fn get_item(&self, id: &ReeInt) -> Option<Arc<Item>> {
        self.storage_items.get(&id).cloned()
    }
    /// Get cached attribute.
    fn get_attr(&self, id: &ReeInt) -> Option<Arc<Attr>> {
        self.storage_attrs.get(&id).cloned()
    }
    /// Get cached effect.
    fn get_effect(&self, id: &ReeInt) -> Option<Arc<Effect>> {
        self.storage_effects.get(&id).cloned()
    }
    /// Get cached mutaplasmid.
    fn get_muta(&self, id: &ReeInt) -> Option<Arc<Muta>> {
        self.storage_mutas.get(&id).cloned()
    }
    /// Get cached warfare buff.
    fn get_buff(&self, id: &ReeInt) -> Option<Arc<Buff>> {
        self.storage_buffs.get(&id).cloned()
    }
    /// Get cached data fingerprint.
    fn get_fingerprint(&self) -> Option<&str> {
        None
    }
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> ch::Result<()> {
        Err(Box::new(IntError::new(
            "RAM-only cache handler does not support persistent cache",
        )))
    }
    /// Update data in handler with passed data.
    fn update_cache(&mut self, ch_data: ch::Data, _: String) {
        move_vec_to_map(ch_data.items, &mut self.storage_items);
        move_vec_to_map(ch_data.attrs, &mut self.storage_attrs);
        move_vec_to_map(ch_data.effects, &mut self.storage_effects);
        move_vec_to_map(ch_data.mutas, &mut self.storage_mutas);
        move_vec_to_map(ch_data.buffs, &mut self.storage_buffs);
    }
}
