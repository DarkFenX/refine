use crate::{
    ac,
    ad::{AEffectId, AItemEffectData},
    util::RMap,
};

pub(super) fn has_online_effect(item_effects: &RMap<AEffectId, AItemEffectData>) -> bool {
    item_effects.contains_key(&ac::effects::ONLINE)
}
pub(super) fn is_turret(item_effects: &RMap<AEffectId, AItemEffectData>) -> bool {
    item_effects.contains_key(&ac::effects::TURRET_FITTED)
}
pub(super) fn is_launcher(item_effects: &RMap<AEffectId, AItemEffectData>) -> bool {
    item_effects.contains_key(&ac::effects::LAUNCHER_FITTED)
}
