use crate::{EffectId, EffectMode};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemEffectInfo {
    pub id: EffectId,
    pub running: bool,
    pub mode: EffectMode,
}
