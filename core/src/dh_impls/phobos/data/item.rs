use crate::{dh, ReeInt};

use super::super::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct Item {
    #[serde(rename = "groupID")]
    pub(in super::super) group_id: ReeInt,
}
impl FsdMerge<dh::Item> for Item {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::Item> {
        vec![dh::Item::new(id, self.group_id)]
    }
}
