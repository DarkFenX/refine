use crate::{
    defs::{EItemCatId, EItemGrpId},
    util::Named,
};

/// EVE item group data.
#[derive(Debug)]
pub struct EItemGroup {
    /// Item group ID.
    pub id: EItemGrpId,
    /// Refers an item category the item group belongs to.
    pub category_id: EItemCatId,
}
impl EItemGroup {
    /// Make a new EVE item group out of passed data.
    pub fn new(id: EItemGrpId, category_id: EItemCatId) -> Self {
        Self { id, category_id }
    }
}
impl Named for EItemGroup {
    fn get_name() -> &'static str {
        "EItemGroup"
    }
}
