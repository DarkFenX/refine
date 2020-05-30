use crate::{defines::ReeInt, util::Named};

/// Item type data.
#[derive(Debug)]
pub struct Item {
    /// Item type ID.
    pub id: ReeInt,
    /// Refers an item group the item type belongs to.
    pub group_id: ReeInt,
    /// Defines if item is dynamic (its attributes are mutable) or not.
    pub is_dynamic: bool,
}
impl Item {
    /// Make a new item type out of passed data.
    pub fn new(id: ReeInt, group_id: ReeInt, is_dynamic: bool) -> Item {
        Item {
            id,
            group_id,
            is_dynamic,
        }
    }
}
impl Named for Item {
    fn get_name() -> &'static str {
        "dh::Item"
    }
}
