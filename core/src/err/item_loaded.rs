use std::{error, fmt};

use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemLoadedError {
    pub item_id: SolItemId,
}
impl ItemLoadedError {
    pub(crate) fn new(item_id: SolItemId) -> Self {
        Self { item_id }
    }
}
impl error::Error for ItemLoadedError {}
impl fmt::Display for ItemLoadedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item {} is not loaded", self.item_id)
    }
}
