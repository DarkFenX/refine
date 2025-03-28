#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HAffectorInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) item_id: rc::ItemId,
    pub(crate) attr_id: Option<rc::AttrId>,
}
impl From<&rc::AffectorInfo> for HAffectorInfo {
    fn from(core_affector_info: &rc::AffectorInfo) -> Self {
        Self {
            item_id: core_affector_info.item_id,
            attr_id: core_affector_info.attr_id,
        }
    }
}
