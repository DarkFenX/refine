use crate::{defs::ReeInt, util::Named};

/// Item group data.
#[derive(Debug)]
pub struct ItemGroup {
    /// Item group ID.
    pub id: ReeInt,
    /// Refers an item category the item group belongs to.
    pub category_id: ReeInt,
}
impl ItemGroup {
    /// Make a new item group out of passed data.
    pub fn new(id: ReeInt, category_id: ReeInt) -> Self {
        Self { id, category_id }
    }
}
impl Named for ItemGroup {
    fn get_name() -> &'static str {
        "edt::ItemGroup"
    }
}
