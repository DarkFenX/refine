use serde_tuple::Serialize_tuple;

use super::strength::HSideEffectStr;

#[derive(Serialize_tuple)]
pub(in crate::info::item::item_booster) struct HSideEffectInfo {
    chance: f64,
    state: bool,
    strength: Option<HSideEffectStr>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSideEffectInfo {
    pub(in crate::info::item::item_booster) fn from_core(mut core_side_effect: rc::FullSideEffectMut) -> Self {
        Self {
            chance: core_side_effect.get_chance().into_f64(),
            state: core_side_effect.get_state(),
            strength: core_side_effect.get_strength().and_then(HSideEffectStr::try_from_core),
        }
    }
}
