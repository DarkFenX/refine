use serde::Deserialize;

use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemList {
    #[serde(rename = "includedTypeIDs", default)]
    pub(in crate::phb) included_item_ids: Vec<i32>,
    #[serde(rename = "includedGroupIDs", default)]
    pub(in crate::phb) included_grp_ids: Vec<i32>,
    #[serde(rename = "includedCategoryIDs", default)]
    pub(in crate::phb) included_cat_ids: Vec<i32>,
    #[serde(rename = "excludedTypeIDs", default)]
    pub(in crate::phb) excluded_item_ids: Vec<i32>,
    #[serde(rename = "excludedGroupIDs", default)]
    pub(in crate::phb) excluded_grp_ids: Vec<i32>,
    #[serde(rename = "excludedCategoryIDs", default)]
    pub(in crate::phb) excluded_cat_ids: Vec<i32>,
}
impl FsdMerge<rc::ed::EItemList> for PItemList {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemList> {
        vec![rc::ed::EItemList {
            id: rc::ed::EItemListId::from_i32(id),
            included_item_ids: self
                .included_item_ids
                .into_iter()
                .map(rc::ed::EItemId::from_i32)
                .collect(),
            included_grp_ids: self
                .included_grp_ids
                .into_iter()
                .map(rc::ed::EItemGrpId::from_i32)
                .collect(),
            included_cat_ids: self
                .included_cat_ids
                .into_iter()
                .map(rc::ed::EItemCatId::from_i32)
                .collect(),
            excluded_item_ids: self
                .excluded_item_ids
                .into_iter()
                .map(rc::ed::EItemId::from_i32)
                .collect(),
            excluded_grp_ids: self
                .excluded_grp_ids
                .into_iter()
                .map(rc::ed::EItemGrpId::from_i32)
                .collect(),
            excluded_cat_ids: self
                .excluded_cat_ids
                .into_iter()
                .map(rc::ed::EItemCatId::from_i32)
                .collect(),
        }]
    }
}
