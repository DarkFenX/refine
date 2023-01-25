use std::fmt;

use crate::{
    ct::{Attr, Buff, Effect, Item, Muta},
    defines::ReeInt,
    util::Result,
};

use super::data::CHData;

/// Cache handler interface definition.
pub trait CacheHandler: fmt::Debug {
    /// Get cached item.
    fn get_item(&self, id: ReeInt) -> Option<&Item>;
    /// Get cached attribute.
    fn get_attr(&self, id: ReeInt) -> Option<&Attr>;
    /// Get cached effect.
    fn get_effect(&self, id: ReeInt) -> Option<&Effect>;
    /// Get cached mutaplasmid.
    fn get_muta(&self, id: ReeInt) -> Option<&Muta>;
    /// Get cached warfare buff.
    fn get_buff(&self, id: ReeInt) -> Option<&Buff>;
    /// Get cached data fingerprint.
    fn get_fingerprint(&self) -> &String;
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> Result<()>;
    /// Update both persistent and memory data in handler with passed data.
    fn update_cache(&mut self, data: CHData, fingerprint: String);
}
