use crate::{
    ed::{EItemCatId, EItemGrpId},
    util::Named,
};

/// EVE item group data.
pub struct EItemGroup {
    /// Item group ID.
    pub id: EItemGrpId,
    /// Refers an item category the item group belongs to.
    pub category_id: EItemCatId,
}
impl Named for EItemGroup {
    fn get_name() -> &'static str {
        "EItemGroup"
    }
}
