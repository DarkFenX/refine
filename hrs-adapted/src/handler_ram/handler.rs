use std::{collections::HashMap, fmt, sync::Arc};

use crate::util::{move_vec_to_map, Error, ErrorKind};

/// Adapted data handler implementation without persistence.
///
/// This handler stores everything only in RAM. Access to data is fast, but has noticeable RAM
/// consumption and adapted data has to be rebuilt every time.
pub struct RamOnlyAdh {
    storage_items: HashMap<rc::ReeInt, Arc<rc::ad::AItem>>,
    storage_attrs: HashMap<rc::ReeInt, Arc<rc::ad::AAttr>>,
    storage_effects: HashMap<rc::ReeInt, Arc<rc::ad::AEffect>>,
    storage_mutas: HashMap<rc::ReeInt, Arc<rc::ad::AMuta>>,
    storage_buffs: HashMap<rc::ReeInt, Arc<rc::ad::ABuff>>,
}
impl RamOnlyAdh {
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
impl fmt::Debug for RamOnlyAdh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RamOnlyAdh()")
    }
}
impl rc::ad::AdaptedDataHandler for RamOnlyAdh {
    /// Get adapted item.
    fn get_item(&self, id: &rc::ReeInt) -> Option<Arc<rc::ad::AItem>> {
        self.storage_items.get(&id).cloned()
    }
    /// Get adapted attribute.
    fn get_attr(&self, id: &rc::ReeInt) -> Option<Arc<rc::ad::AAttr>> {
        self.storage_attrs.get(&id).cloned()
    }
    /// Get adapted effect.
    fn get_effect(&self, id: &rc::ReeInt) -> Option<Arc<rc::ad::AEffect>> {
        self.storage_effects.get(&id).cloned()
    }
    /// Get adapted mutaplasmid.
    fn get_muta(&self, id: &rc::ReeInt) -> Option<Arc<rc::ad::AMuta>> {
        self.storage_mutas.get(&id).cloned()
    }
    /// Get adapted warfare buff.
    fn get_buff(&self, id: &rc::ReeInt) -> Option<Arc<rc::ad::ABuff>> {
        self.storage_buffs.get(&id).cloned()
    }
    /// Get adapted data fingerprint.
    fn get_fingerprint(&self) -> Option<&str> {
        None
    }
    /// Load cache from persistent storage.
    ///
    /// Will always fail, since this handler does not implement persistent storage.
    fn load_cache(&mut self) -> rc::ad::AResult<()> {
        Err(Error::new(ErrorKind::NoCacheSupport).into())
    }
    /// Update handler with passed adapted data.
    fn update_data(&mut self, adata: rc::ad::AData, _: String) {
        move_vec_to_map(adata.items, &mut self.storage_items);
        move_vec_to_map(adata.attrs, &mut self.storage_attrs);
        move_vec_to_map(adata.effects, &mut self.storage_effects);
        move_vec_to_map(adata.mutas, &mut self.storage_mutas);
        move_vec_to_map(adata.buffs, &mut self.storage_buffs);
    }
}
