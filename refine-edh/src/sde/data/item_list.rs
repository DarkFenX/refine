use serde::Deserialize;

use crate::sde::data::ExtractOne;

#[derive(Deserialize)]
pub(in crate::sde) struct SItemList {
    #[serde(rename = "_key")]
    id: i32,
    #[serde(rename = "includedTypeIDs", default)]
    included_type_ids: Vec<i32>,
    #[serde(rename = "includedGroupIDs", default)]
    included_group_ids: Vec<i32>,
    #[serde(rename = "includedCategoryIDs", default)]
    included_category_ids: Vec<i32>,
    #[serde(rename = "excludedTypeIDs", default)]
    excluded_type_ids: Vec<i32>,
    #[serde(rename = "excludedGroupIDs", default)]
    excluded_group_ids: Vec<i32>,
    #[serde(rename = "excludedCategoryIDs", default)]
    excluded_category_ids: Vec<i32>,
}
impl ExtractOne<rc::ed::EItemList> for SItemList {
    fn extract(self) -> Vec<rc::ed::EItemList> {
        vec![rc::ed::EItemList {
            id: rc::ed::EItemListId::from_i32(self.id),
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
