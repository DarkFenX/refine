use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PItem {
    #[serde(rename = "groupID")]
    pub(in crate::phb) group_id: rc::ReeInt,
}
impl FsdMerge<rc::ed::EItem> for PItem {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EItem> {
        vec![rc::ed::EItem::new(id, self.group_id)]
    }
}
