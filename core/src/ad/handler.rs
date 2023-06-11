use std::fmt;

use crate::defs::ReeInt;

use super::{
    data::{AData, ArcAttr, ArcBuff, ArcEffect, ArcItem, ArcMuta},
    AResult,
};

/// Adapted data handler interface definition.
///
/// Primary role of an adapted data handler implementation is to provide adapted data by request of
/// the library. Additionally, it can persist adapted types somewhere to avoid regeneration of
/// adapted data on every run.
pub trait AdaptedDataHandler: fmt::Debug + Send + Sync {
    /// Get adapted item.
    fn get_item(&self, id: &ReeInt) -> Option<ArcItem>;
    /// Get adapted attribute.
    fn get_attr(&self, id: &ReeInt) -> Option<ArcAttr>;
    /// Get adapted effect.
    fn get_effect(&self, id: &ReeInt) -> Option<ArcEffect>;
    /// Get adapted mutaplasmid.
    fn get_muta(&self, id: &ReeInt) -> Option<ArcMuta>;
    /// Get adapted warfare buff.
    fn get_buff(&self, id: &ReeInt) -> Option<ArcBuff>;
    /// Get adapted data fingerprint.
    fn get_data_fingerprint(&self) -> Option<&str>;
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> AResult<()>;
    /// Update data in handler with passed data.
    fn update_data(&mut self, data: AData, fingerprint: String);
}
