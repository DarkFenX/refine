use crate::{
    ed::{EItemCatId, EItemGrpId, EItemId, EItemListId},
    util::Named,
};

/// EVE item type list data.
pub struct EItemList {
    /// Item list ID.
    pub id: EItemListId,
    /// Included item type IDs.
    pub included_item_ids: Vec<EItemId>,
    /// Included item group IDs.
    pub included_grp_ids: Vec<EItemGrpId>,
    /// Included item category IDs.
    pub included_cat_ids: Vec<EItemCatId>,
    /// Excluded item type IDs.
    pub excluded_item_ids: Vec<EItemId>,
    /// Excluded item group IDs.
    pub excluded_grp_ids: Vec<EItemGrpId>,
    /// Excluded item category IDs.
    pub excluded_cat_ids: Vec<EItemCatId>,
}
impl Named for EItemList {
    fn get_name() -> &'static str {
        "EItemList"
    }
}
