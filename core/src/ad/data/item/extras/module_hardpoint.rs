use crate::{
    ac,
    ad::{AEffectId, AItemEffectData},
    util::StMap,
};

pub(super) fn is_turret(item_effects: &StMap<AEffectId, AItemEffectData>) -> bool {
    item_effects.keys().any(|v| v == &ac::effects::TURRET_FITTED)
}
pub(super) fn is_launcher(item_effects: &StMap<AEffectId, AItemEffectData>) -> bool {
    item_effects.keys().any(|v| v == &ac::effects::LAUNCHER_FITTED)
}
