use std::{fmt, sync::Arc};

use crate::{
    defs::ReeInt,
    ert::{Attr, Buff, Effect, Item, Muta},
};

use super::{data::Data, Result};

/// Cache handler interface definition.
pub trait CacheHandler: fmt::Debug + Send + Sync {
    /// Get cached item.
    fn get_item(&self, id: &ReeInt) -> Option<Arc<Item>>;
    /// Get cached attribute.
    fn get_attr(&self, id: &ReeInt) -> Option<Arc<Attr>>;
    /// Get cached effect.
    fn get_effect(&self, id: &ReeInt) -> Option<Arc<Effect>>;
    /// Get cached mutaplasmid.
    fn get_muta(&self, id: &ReeInt) -> Option<Arc<Muta>>;
    /// Get cached warfare buff.
    fn get_buff(&self, id: &ReeInt) -> Option<Arc<Buff>>;
    /// Get cached data fingerprint.
    fn get_fingerprint(&self) -> Option<&str>;
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> Result<()>;
    /// Update data in handler with passed data.
    fn update_cache(&mut self, ch_data: Data, fingerprint: String);
}
