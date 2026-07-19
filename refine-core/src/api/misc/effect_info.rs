use crate::misc::EffectMode;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
pub struct EffectInfo {
    pub running: bool,
    pub mode: EffectMode,
}
