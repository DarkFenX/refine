use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PItem {
    #[serde(rename = "groupID")]
    pub(in crate::phb) group_id: rc::EItemGrpId,
    pub(in crate::phb) capacity: rc::AttrVal,
    pub(in crate::phb) mass: rc::AttrVal,
    pub(in crate::phb) radius: rc::AttrVal,
    pub(in crate::phb) volume: rc::AttrVal,
}
impl FsdMerge<rc::ed::EItem> for PItem {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItem> {
        vec![rc::ed::EItem::new(
            id,
            self.group_id,
            self.capacity,
            self.mass,
            self.radius,
            self.volume,
        )]
    }
}
