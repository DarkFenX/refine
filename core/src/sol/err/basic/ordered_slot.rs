use crate::{
    defs::{Idx, SolItemId},
    sol::SolModRack,
};

#[derive(Debug)]
pub struct OrderedSlotError {
    pub rack: SolModRack,
    pub position: Idx,
    pub item_id: SolItemId,
}
impl OrderedSlotError {
    pub(crate) fn new(rack: SolModRack, position: Idx, item_id: SolItemId) -> Self {
        Self {
            rack,
            position,
            item_id,
        }
    }
}
impl std::error::Error for OrderedSlotError {}
impl std::fmt::Display for OrderedSlotError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} slot {} is occupied by item {}",
            self.rack, self.position, self.item_id
        )
    }
}
