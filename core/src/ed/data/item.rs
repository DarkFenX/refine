use crate::{defs::ReeInt, util::Named};

/// EVE item type data.
#[derive(Debug)]
pub struct EItem {
    /// Item type ID.
    pub id: ReeInt,
    /// Refers an item group the item type belongs to.
    pub group_id: ReeInt,
}
impl EItem {
    /// Make a new EVE item type out of passed data.
    pub fn new(id: ReeInt, group_id: ReeInt) -> Self {
        Self { id, group_id }
    }
}
impl Named for EItem {
    fn get_name() -> &'static str {
        "ed::EItem"
    }
}
