use crate::{defs::ReeInt, edh_impls::phobos::fsd::FsdMerge, edt};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemGroup {
    #[serde(rename = "categoryID")]
    pub(in super::super) category_id: ReeInt,
}
impl FsdMerge<edt::ItemGroup> for ItemGroup {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::ItemGroup> {
        vec![edt::ItemGroup::new(id, self.category_id)]
    }
}
