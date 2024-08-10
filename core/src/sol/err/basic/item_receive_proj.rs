use std::{error, fmt};

use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ItemReceiveProjError {
    pub item_id: SolItemId,
    pub item_kind: &'static str,
}
impl ItemReceiveProjError {
    pub(crate) fn new(item_id: SolItemId, item_kind: &'static str) -> Self {
        Self { item_id, item_kind }
    }
}
impl error::Error for ItemReceiveProjError {}
impl fmt::Display for ItemReceiveProjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} not found", self.item_kind, self.item_id)
    }
}
