use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItem {
    #[serde(rename = "groupID")]
    pub(in crate::phb) group_id: i32,
    pub(in crate::phb) capacity: f64,
    pub(in crate::phb) mass: f64,
    pub(in crate::phb) radius: f64,
    pub(in crate::phb) volume: f64,
}
impl FsdMerge<rc::ed::EItem> for PItem {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItem> {
        vec![rc::ed::EItem {
            id: rc::ed::EItemId::from_i32(id),
            group_id: rc::ed::EItemGrpId::from_i32(self.group_id),
            capacity: rc::ed::EFloat::from_f64(self.capacity),
            mass: rc::ed::EFloat::from_f64(self.mass),
            radius: rc::ed::EFloat::from_f64(self.radius),
            volume: rc::ed::EFloat::from_f64(self.volume),
        }]
    }
}
