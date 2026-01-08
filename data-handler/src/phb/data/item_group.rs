use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemGroup {
    #[serde(rename = "categoryID")]
    pub(in crate::phb) category_id: i32,
}
impl FsdMerge<rc::ed::EItemGroup> for PItemGroup {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemGroup> {
        vec![rc::ed::EItemGroup {
            id: rc::ed::EItemGrpId::from_i32(id),
            category_id: rc::ed::EItemCatId::from_i32(self.category_id),
        }]
    }
}
