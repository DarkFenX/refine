use crate::shared::HAttrId;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::item::extended::modification) struct HAffector {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::ItemId,
    attr_id: Option<HAttrId>,
}
impl From<&rc::Affector> for HAffector {
    fn from(core_affector: &rc::Affector) -> Self {
        Self {
            item_id: core_affector.item_id,
            attr_id: core_affector.attr_id.map(Into::into),
        }
    }
}
