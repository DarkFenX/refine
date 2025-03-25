use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemList {
    #[serde(rename = "includedTypeIDs")]
    pub(in crate::phb) included_item_ids: Vec<rc::ed::EItemId>,
    #[serde(rename = "includedGroupIDs")]
    pub(in crate::phb) included_grp_ids: Vec<rc::ed::EItemGrpId>,
    #[serde(rename = "includedCategoryIDs")]
    pub(in crate::phb) included_cat_ids: Vec<rc::ed::EItemCatId>,
    #[serde(rename = "excludedTypeIDs")]
    pub(in crate::phb) excluded_item_ids: Vec<rc::ed::EItemId>,
    #[serde(rename = "excludedGroupIDs")]
    pub(in crate::phb) excluded_grp_ids: Vec<rc::ed::EItemGrpId>,
    #[serde(rename = "excludedCategoryIDs")]
    pub(in crate::phb) excluded_cat_ids: Vec<rc::ed::EItemCatId>,
}
impl FsdMerge<rc::ed::EItemList> for PItemList {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemList> {
        vec![rc::ed::EItemList {
            id,
            included_item_ids: self.included_item_ids,
            included_grp_ids: self.included_grp_ids,
            included_cat_ids: self.included_cat_ids,
            excluded_item_ids: self.excluded_item_ids,
            excluded_grp_ids: self.excluded_grp_ids,
            excluded_cat_ids: self.excluded_cat_ids,
        }]
    }
}
