use crate::ad::{AAbilId, AEffectId};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AAbil {
    pub id: AAbilId,
    pub effect_id: AEffectId,
}
