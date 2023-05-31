use crate::{defs::ReeInt, edh_impls::phobos::fsd::FsdMerge, edt};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct Item {
    #[serde(rename = "groupID")]
    pub(in super::super) group_id: ReeInt,
}
impl FsdMerge<edt::Item> for Item {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::Item> {
        vec![edt::Item::new(id, self.group_id)]
    }
}
