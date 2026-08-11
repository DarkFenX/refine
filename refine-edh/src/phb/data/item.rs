use serde::Deserialize;

use crate::phb::data::{Key, KeyMergeOne};

#[derive(Deserialize)]
pub(in crate::phb) struct PItem {
    #[serde(rename = "groupID")]
    group_id: i32,
    capacity: f64,
    mass: f64,
    radius: f64,
    volume: f64,
}
impl KeyMergeOne<rc::ed::EItem> for PItem {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItem> {
        vec![rc::ed::EItem {
            id: rc::ed::EItemId::from_i32(key),
            group_id: rc::ed::EItemGrpId::from_i32(self.group_id),
            capacity: Some(rc::ed::EFloat::from_f64(self.capacity)),
            mass: Some(rc::ed::EFloat::from_f64(self.mass)),
            radius: Some(rc::ed::EFloat::from_f64(self.radius)),
            volume: Some(rc::ed::EFloat::from_f64(self.volume)),
        }]
    }
}
