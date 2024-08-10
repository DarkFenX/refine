use std::{error, fmt};

use crate::defs::SolItemId;

#[derive(Debug)]
pub struct ChargeFoundError {
    pub cont_item_id: SolItemId,
}
impl ChargeFoundError {
    pub(crate) fn new(cont_item_id: SolItemId) -> Self {
        Self { cont_item_id }
    }
}
impl error::Error for ChargeFoundError {}
impl fmt::Display for ChargeFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item {} does not have charge set", self.cont_item_id)
    }
}
