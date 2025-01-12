#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HAffectorInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) item_id: rc::SolItemId,
    pub(crate) attr_id: Option<rc::EAttrId>,
}
impl HAffectorInfo {
    fn new(item_id: rc::SolItemId, attr_id: Option<rc::EAttrId>) -> Self {
        Self { item_id, attr_id }
    }
}
impl From<&rc::SolAffectorInfo> for HAffectorInfo {
    fn from(core_affector_info: &rc::SolAffectorInfo) -> Self {
        Self::new(core_affector_info.item_id, core_affector_info.attr_id)
    }
}
