use crate::{
    ed::{EItemCatId, EItemGrpId, EItemId, EItemListId},
    util::LibNamed,
};

pub struct EItemList {
    pub id: EItemListId,
    pub included_item_ids: Vec<EItemId>,
    pub included_grp_ids: Vec<EItemGrpId>,
    pub included_cat_ids: Vec<EItemCatId>,
    pub excluded_item_ids: Vec<EItemId>,
    pub excluded_grp_ids: Vec<EItemGrpId>,
    pub excluded_cat_ids: Vec<EItemCatId>,
}
impl LibNamed for EItemList {
    fn lib_get_name() -> &'static str {
        "EItemList"
    }
}
