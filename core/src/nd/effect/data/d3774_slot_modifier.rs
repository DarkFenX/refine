use crate::{
    ad::{AAttrId, AEffect, AEffectId},
    ed::EEffectId,
    nd::{NEffect, effect::data::shared::mods::mk_subsystem_mod},
};

const EFFECT_EID: EEffectId = EEffectId::SLOT_MODIFIER;
const EFFECT_AID: AEffectId = AEffectId::SLOT_MODIFIER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(internal_update_effect),
        ..
    }
}

fn internal_update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: slot modifier effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_subsystem_mod(AAttrId::HI_SLOT_MODIFIER, AAttrId::HI_SLOTS),
        mk_subsystem_mod(AAttrId::MED_SLOT_MODIFIER, AAttrId::MED_SLOTS),
        mk_subsystem_mod(AAttrId::LOW_SLOT_MODIFIER, AAttrId::LOW_SLOTS),
    ]);
}
