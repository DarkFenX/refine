use std::fmt;

use crate::{
    ct::{Attr, Buff, Effect, Item, Muta},
    defines::ReeInt,
    util::Result,
};

use super::container::Container;

/// Cache handler interface definition.
pub trait CacheHandler: fmt::Debug {
    // /// Get cached item.
    // fn get_item(&self, id: ReeInt) -> Result<Item>;
    // /// Get cached attribute.
    // fn get_attr(&self, id: ReeInt) -> Result<Attr>;
    // /// Get cached effect.
    // fn get_effect(&self, id: ReeInt) -> Result<Effect>;
    // /// Get cached mutaplasmid.
    // fn get_muta(&self, id: ReeInt) -> Result<Muta>;
    // /// Get cached warfare buff.
    // fn get_buff(&self, id: ReeInt) -> Result<Buff>;
    // /// Get cached data fingerprint.
    // fn get_fingerprint(&self) -> Result<String>;
    /// Update cache.
    fn update_cache(&mut self, data: Container, fingerprint: String);
}
