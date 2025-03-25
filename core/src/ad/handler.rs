use std::fmt;

use crate::ad::{AAttrId, ABuffId, AData, AEffectId, AItemId, AResult, ArcAttr, ArcBuff, ArcEffect, ArcItem, ArcMuta};

/// Adapted data handler interface definition.
///
/// Primary role of an adapted data handler implementation is to provide adapted data by request of
/// the library. Additionally, it can persist adapted types somewhere to avoid regeneration of
/// adapted data on every run.
pub trait AdaptedDataHandler: fmt::Debug + Send + Sync {
    /// Get adapted item.
    fn get_item(&self, id: &AItemId) -> Option<&ArcItem>;
    /// Get adapted attribute.
    fn get_attr(&self, id: &AAttrId) -> Option<&ArcAttr>;
    /// Get adapted effect.
    fn get_effect(&self, id: &AEffectId) -> Option<&ArcEffect>;
    /// Get adapted mutator.
    fn get_mutator(&self, id: &AItemId) -> Option<&ArcMuta>;
    /// Get adapted warfare buff.
    fn get_buff(&self, id: &ABuffId) -> Option<&ArcBuff>;
    /// Get adapted data fingerprint.
    fn get_data_fingerprint(&self) -> Option<String>;
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> AResult<()>;
    /// Update data in handler with passed data.
    fn update_data(&mut self, data: AData, fingerprint: String);
    /// Get adapted handler version.
    ///
    /// Change in adapted handler version triggers adapted data cache rebuild, even if source data
    /// and core library version stayed the same.
    fn get_handler_version(&self) -> String;
}
