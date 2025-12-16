use crate::shared::HAttrId;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::item::extended::modification) struct HAffectorInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    attr_id: Option<HAttrId>,
}
impl From<&rc::AffectorInfo> for HAffectorInfo {
    fn from(core_affector_info: &rc::AffectorInfo) -> Self {
        Self {
            item_id: core_affector_info.item_id,
            attr_id: core_affector_info.attr_id.map(Into::into),
        }
    }
}
