use crate::EffectMode;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemEffectInfo {
    pub running: bool,
    pub mode: EffectMode,
}
