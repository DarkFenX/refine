use serde::Deserialize;

use crate::phb::parsing::{Key, KeyMergeOne};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemGroup {
    #[serde(rename = "categoryID")]
    pub(in crate::phb) category_id: i32,
}
impl KeyMergeOne<rc::ed::EItemGroup> for PItemGroup {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItemGroup> {
        vec![rc::ed::EItemGroup {
            id: rc::ed::EItemGrpId::from_i32(key),
            category_id: rc::ed::EItemCatId::from_i32(self.category_id),
        }]
    }
}
