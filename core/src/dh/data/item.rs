use crate::{util::Named, ReeInt};

/// Item type data.
#[derive(Debug)]
pub struct Item {
    /// Item type ID.
    pub id: ReeInt,
    /// Refers an item group the item type belongs to.
    pub group_id: ReeInt,
}
impl Item {
    /// Make a new item type out of passed data.
    pub fn new(id: ReeInt, group_id: ReeInt) -> Self {
        Self { id, group_id }
    }
}
impl Named for Item {
    fn get_name() -> &'static str {
        "dh::Item"
    }
}
