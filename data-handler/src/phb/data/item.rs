use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItem {
    #[serde(rename = "groupID")]
    pub(in crate::phb) group_id: rc::ed::EItemGrpId,
    pub(in crate::phb) capacity: rc::ed::EAttrVal,
    pub(in crate::phb) mass: rc::ed::EAttrVal,
    pub(in crate::phb) radius: rc::ed::EAttrVal,
    pub(in crate::phb) volume: rc::ed::EAttrVal,
}
impl FsdMerge<rc::ed::EItem> for PItem {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItem> {
        vec![rc::ed::EItem {
            id,
            group_id: self.group_id,
            capacity: self.capacity,
            mass: self.mass,
            radius: self.radius,
            volume: self.volume,
        }]
    }
}
