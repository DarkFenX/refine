use crate::defines::ReeInt;
use crate::dh;

use super::super::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemGroup {
    #[serde(rename = "categoryID")]
    pub(in super::super) category_id: ReeInt,
}
impl FsdMerge<dh::ItemGroup> for ItemGroup {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemGroup> {
        vec![dh::ItemGroup::new(id, self.category_id)]
    }
}
