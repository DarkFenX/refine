use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemGroup {
    #[serde(rename = "categoryID")]
    pub(crate) category_id: rc::ReeInt,
}
impl FsdMerge<rc::edt::ItemGroup> for ItemGroup {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::ItemGroup> {
        vec![rc::edt::ItemGroup::new(id, self.category_id)]
    }
}
