use crate::{
    defs::{ItemCatId, ItemGrpId},
    util::Named,
};

/// EVE item group data.
#[derive(Debug)]
pub struct EItemGroup {
    /// Item group ID.
    pub id: ItemGrpId,
    /// Refers an item category the item group belongs to.
    pub category_id: ItemCatId,
}
impl EItemGroup {
    /// Make a new EVE item group out of passed data.
    pub fn new(id: ItemGrpId, category_id: ItemCatId) -> Self {
        Self { id, category_id }
    }
}
impl Named for EItemGroup {
    fn get_name() -> &'static str {
        "EItemGroup"
    }
}
