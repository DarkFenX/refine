use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PItemGroup {
    #[serde(rename = "categoryID")]
    pub(in crate::phb) category_id: rc::ReeInt,
}
impl FsdMerge<rc::ed::EItemGroup> for PItemGroup {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EItemGroup> {
        vec![rc::ed::EItemGroup::new(id, self.category_id)]
    }
}
