use serde::Deserialize;

use crate::phb::data::{Key, KeyMergeOne};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemList {
    #[serde(rename = "includedTypeIDs", default)]
    pub(in crate::phb) included_type_ids: Vec<i32>,
    #[serde(rename = "includedGroupIDs", default)]
    pub(in crate::phb) included_group_ids: Vec<i32>,
    #[serde(rename = "includedCategoryIDs", default)]
    pub(in crate::phb) included_category_ids: Vec<i32>,
    #[serde(rename = "excludedTypeIDs", default)]
    pub(in crate::phb) excluded_type_ids: Vec<i32>,
    #[serde(rename = "excludedGroupIDs", default)]
    pub(in crate::phb) excluded_group_ids: Vec<i32>,
    #[serde(rename = "excludedCategoryIDs", default)]
    pub(in crate::phb) excluded_category_ids: Vec<i32>,
}
impl KeyMergeOne<rc::ed::EItemList> for PItemList {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItemList> {
        vec![rc::ed::EItemList {
            id: rc::ed::EItemListId::from_i32(key),
            included_item_ids: self
                .included_type_ids
                .into_iter()
                .map(rc::ed::EItemId::from_i32)
                .collect(),
            included_grp_ids: self
                .included_group_ids
                .into_iter()
                .map(rc::ed::EItemGrpId::from_i32)
                .collect(),
            included_cat_ids: self
                .included_category_ids
                .into_iter()
                .map(rc::ed::EItemCatId::from_i32)
                .collect(),
            excluded_item_ids: self
                .excluded_type_ids
                .into_iter()
                .map(rc::ed::EItemId::from_i32)
                .collect(),
            excluded_grp_ids: self
                .excluded_group_ids
                .into_iter()
                .map(rc::ed::EItemGrpId::from_i32)
                .collect(),
            excluded_cat_ids: self
                .excluded_category_ids
                .into_iter()
                .map(rc::ed::EItemCatId::from_i32)
                .collect(),
        }]
    }
}
