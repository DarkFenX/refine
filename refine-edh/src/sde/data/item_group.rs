use serde::Deserialize;

use crate::sde::data::ExtractOne;

#[derive(Deserialize)]
pub(in crate::sde) struct SItemGroup {
    #[serde(rename = "_key")]
    id: i32,
    #[serde(rename = "categoryID")]
    category_id: i32,
}
impl ExtractOne<rc::ed::EItemGroup> for SItemGroup {
    fn extract(self) -> Vec<rc::ed::EItemGroup> {
        vec![rc::ed::EItemGroup {
            id: rc::ed::EItemGrpId::from_i32(self.id),
            category_id: rc::ed::EItemCatId::from_i32(self.category_id),
        }]
    }
}
