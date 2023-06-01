use std::{fmt, sync::Arc};

use crate::{
    adt::{Attr, Buff, Effect, Item, Muta},
    defs::ReeInt,
};

use super::{data::Data, Result};

/// Adapted data handler interface definition.
pub trait AdaptedDataHandler: fmt::Debug + Send + Sync {
    /// Get adapted item.
    fn get_item(&self, id: &ReeInt) -> Option<Arc<Item>>;
    /// Get adapted attribute.
    fn get_attr(&self, id: &ReeInt) -> Option<Arc<Attr>>;
    /// Get adapted effect.
    fn get_effect(&self, id: &ReeInt) -> Option<Arc<Effect>>;
    /// Get adapted mutaplasmid.
    fn get_muta(&self, id: &ReeInt) -> Option<Arc<Muta>>;
    /// Get adapted warfare buff.
    fn get_buff(&self, id: &ReeInt) -> Option<Arc<Buff>>;
    /// Get adapted data fingerprint.
    fn get_fingerprint(&self) -> Option<&str>;
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> Result<()>;
    /// Update data in handler with passed data.
    fn update_data(&mut self, adata: Data, fingerprint: String);
}
