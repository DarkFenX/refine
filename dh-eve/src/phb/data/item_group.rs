use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemGroup {
    #[serde(rename = "categoryID")]
    pub(in crate::phb) category_id: rc::EItemCatId,
}
impl FsdMerge<rc::ed::EItemGroup> for PItemGroup {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemGroup> {
        vec![rc::ed::EItemGroup::new(id, self.category_id)]
    }
}
