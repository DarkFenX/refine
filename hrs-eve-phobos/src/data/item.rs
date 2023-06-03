use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Item {
    #[serde(rename = "groupID")]
    pub(crate) group_id: rc::ReeInt,
}
impl FsdMerge<rc::ed::EItem> for Item {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EItem> {
        vec![rc::ed::EItem::new(id, self.group_id)]
    }
}
