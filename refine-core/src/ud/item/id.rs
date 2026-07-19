use std::num::Wrapping;

use crate::util::{LibDefault, LibIncrement};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct ItemId(u32);
impl LibDefault for ItemId {
    fn lib_default() -> Self {
        Self(0)
    }
}
impl LibIncrement for ItemId {
    fn lib_increment(&mut self) {
        self.0 = (Wrapping(self.0) + Wrapping(1)).0;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Error
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(thiserror::Error, Debug)]
#[error("item {item_id} not found")]
pub struct ItemFoundError {
    pub item_id: ItemId,
}
// Conversion needed for unified user entity container to work
impl From<ItemId> for ItemFoundError {
    fn from(item_id: ItemId) -> Self {
        Self { item_id }
    }
}
