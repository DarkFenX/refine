use crate::{ad, util::Named};

// Represents an item (or item type, according to EVE terminology).
//
// An item carries alot of info needed to calculate fit attributes, for example base attribute
// values.
pub(crate) struct RItem {
    a_item: ad::AItem,
}
impl RItem {
    pub(crate) fn new(a_item: ad::AItem) -> Self {
        Self { a_item }
    }
}
impl Named for RItem {
    fn get_name() -> &'static str {
        "RItem"
    }
}
