use std::{fmt, sync::Arc};

use crate::{
    adt::{AAttr, ABuff, AEffect, AItem, AMuta},
    defs::ReeInt,
};

use super::{data::AData, Result};

/// Adapted data handler interface definition.
pub trait AdaptedDataHandler: fmt::Debug + Send + Sync {
    /// Get adapted item.
    fn get_item(&self, id: &ReeInt) -> Option<Arc<AItem>>;
    /// Get adapted attribute.
    fn get_attr(&self, id: &ReeInt) -> Option<Arc<AAttr>>;
    /// Get adapted effect.
    fn get_effect(&self, id: &ReeInt) -> Option<Arc<AEffect>>;
    /// Get adapted mutaplasmid.
    fn get_muta(&self, id: &ReeInt) -> Option<Arc<AMuta>>;
    /// Get adapted warfare buff.
    fn get_buff(&self, id: &ReeInt) -> Option<Arc<ABuff>>;
    /// Get adapted data fingerprint.
    fn get_fingerprint(&self) -> Option<&str>;
    /// Load cache from persistent storage.
    fn load_cache(&mut self) -> Result<()>;
    /// Update data in handler with passed data.
    fn update_data(&mut self, adata: AData, fingerprint: String);
}
