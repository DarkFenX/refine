use crate::ad::{ABuffAggrMode, ABuffId, ABuffModifiers, AOp};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct ABuff {
    pub id: ABuffId,
    pub aggr_mode: ABuffAggrMode,
    pub op: AOp,
    pub mods: ABuffModifiers,
}
