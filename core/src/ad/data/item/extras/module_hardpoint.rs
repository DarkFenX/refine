use crate::{
    ac,
    ad::{AEffectId, AItemEffectData},
    util::RMap,
};

pub(super) fn is_turret(item_effects: &RMap<AEffectId, AItemEffectData>) -> bool {
    item_effects.keys().any(|v| v == &ac::effects::TURRET_FITTED)
}
pub(super) fn is_launcher(item_effects: &RMap<AEffectId, AItemEffectData>) -> bool {
    item_effects.keys().any(|v| v == &ac::effects::LAUNCHER_FITTED)
}
