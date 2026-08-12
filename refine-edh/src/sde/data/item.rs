use serde::Deserialize;

use crate::sde::data::ExtractOne;

#[derive(Deserialize)]
pub(in crate::sde) struct SItem {
    #[serde(rename = "_key")]
    id: i32,
    #[serde(rename = "groupID")]
    group_id: i32,
    capacity: Option<f64>,
    mass: Option<f64>,
    radius: Option<f64>,
    volume: Option<f64>,
}
impl ExtractOne<rc::ed::EItem> for SItem {
    fn extract(self) -> Vec<rc::ed::EItem> {
        vec![rc::ed::EItem {
            id: rc::ed::EItemId::from_i32(self.id),
            group_id: rc::ed::EItemGrpId::from_i32(self.group_id),
            capacity: self.capacity.map(rc::ed::EFloat::from_f64),
            mass: self.mass.map(rc::ed::EFloat::from_f64),
            radius: self.radius.map(rc::ed::EFloat::from_f64),
            volume: self.volume.map(rc::ed::EFloat::from_f64),
        }]
    }
}
