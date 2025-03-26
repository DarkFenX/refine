use std::fmt;

use crate::{
    VERSION,
    util::{Error, move_map_to_arcmap},
};

/// Adapted data handler implementation without persistence.
///
/// This handler stores everything only in RAM. Access to data is fast, but has noticeable RAM
/// consumption and adapted data has to be rebuilt every time.
pub struct RamOnlyAdh {
    storage_items: rc::util::StMap<rc::ad::AItemId, rc::ad::ArcItem>,
    storage_attrs: rc::util::StMap<rc::ad::AAttrId, rc::ad::ArcAttr>,
    storage_effects: rc::util::StMap<rc::ad::AEffectId, rc::ad::ArcEffect>,
    storage_mutas: rc::util::StMap<rc::ad::AItemId, rc::ad::ArcMuta>,
    storage_buffs: rc::util::StMap<rc::ad::ABuffId, rc::ad::ArcBuff>,
}
impl RamOnlyAdh {
    pub fn new() -> Self {
        Self {
            storage_items: rc::util::StMap::new(),
            storage_attrs: rc::util::StMap::new(),
            storage_effects: rc::util::StMap::new(),
            storage_mutas: rc::util::StMap::new(),
            storage_buffs: rc::util::StMap::new(),
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
    fn get_item(&self, id: &rc::ad::AItemId) -> Option<&rc::ad::ArcItem> {
        self.storage_items.get(id)
    }
    /// Get adapted attribute.
    fn get_attr(&self, id: &rc::ad::AAttrId) -> Option<&rc::ad::ArcAttr> {
        self.storage_attrs.get(id)
    }
    /// Get adapted effect.
    fn get_effect(&self, id: &rc::ad::AEffectId) -> Option<&rc::ad::ArcEffect> {
        self.storage_effects.get(id)
    }
    /// Get adapted mutator.
    fn get_mutator(&self, id: &rc::ad::AItemId) -> Option<&rc::ad::ArcMuta> {
        self.storage_mutas.get(id)
    }
    /// Get adapted warfare buff.
    fn get_buff(&self, id: &rc::ad::ABuffId) -> Option<&rc::ad::ArcBuff> {
        self.storage_buffs.get(id)
    }
    /// Get adapted data fingerprint.
    ///
    /// Always return None, since it does not persist data and does not store fingerprint.
    fn get_data_fingerprint(&self) -> Option<String> {
        None
    }
    /// Load cache from persistent storage.
    ///
    /// Will always fail, since this handler does not implement persistent storage.
    fn load_cache(&mut self) -> rc::ad::AResult<()> {
        Err(Error::NoCacheSupport.into())
    }
    /// Update handler with passed adapted data.
    fn update_data(&mut self, a_data: rc::ad::AData, _: String) {
        move_map_to_arcmap(a_data.items, &mut self.storage_items);
        move_map_to_arcmap(a_data.attrs, &mut self.storage_attrs);
        move_map_to_arcmap(a_data.effects, &mut self.storage_effects);
        move_map_to_arcmap(a_data.mutas, &mut self.storage_mutas);
        move_map_to_arcmap(a_data.buffs, &mut self.storage_buffs);
    }
    /// Get adapted handler version.
    ///
    /// Change in adapted handler version triggers adapted data cache rebuild, even if source data
    /// and core library version stayed the same.
    fn get_handler_version(&self) -> String {
        VERSION.to_string()
    }
}
impl Default for RamOnlyAdh {
    fn default() -> Self {
        Self::new()
    }
}
