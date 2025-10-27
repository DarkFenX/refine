use crate::{
    ac,
    ad::{AEffect, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, eff::shared::prop_mods::mk_a_modifier_sig},
};

const E_EFFECT_ID: EEffectId = ec::effects::MICRO_JUMP_DRIVE;
const A_EFFECT_ID: AEffectId = ac::effects::MICRO_JUMP_DRIVE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: MJD effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.push(mk_a_modifier_sig());
}
