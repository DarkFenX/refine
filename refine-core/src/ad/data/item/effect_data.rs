use crate::ad::{AAttrId, ACount, AValue};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AItemEffectData {
    pub cooldown: Option<AValue>,
    pub charge_count: Option<ACount>,
    pub charge_reload_duration: Option<AValue>,
    pub autocharge_attr_id: Option<AAttrId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemEffectData {
    pub(in crate::ad::data::item) const fn default() -> Self {
        AItemEffectData {
            cooldown: None,
            charge_count: None,
            charge_reload_duration: None,
            autocharge_attr_id: None,
        }
    }
}
