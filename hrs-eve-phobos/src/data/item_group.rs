use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemGroup {
    #[serde(rename = "categoryID")]
    pub(crate) category_id: rc::ReeInt,
}
impl FsdMerge<rc::edt::EItemGroup> for ItemGroup {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::EItemGroup> {
        vec![rc::edt::EItemGroup::new(id, self.category_id)]
    }
}
