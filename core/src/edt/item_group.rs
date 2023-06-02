use crate::{defs::ReeInt, util::Named};

/// EVE item group data.
#[derive(Debug)]
pub struct EItemGroup {
    /// Item group ID.
    pub id: ReeInt,
    /// Refers an item category the item group belongs to.
    pub category_id: ReeInt,
}
impl EItemGroup {
    /// Make a new EVE item group out of passed data.
    pub fn new(id: ReeInt, category_id: ReeInt) -> Self {
        Self { id, category_id }
    }
}
impl Named for EItemGroup {
    fn get_name() -> &'static str {
        "edt::EItemGroup"
    }
}
