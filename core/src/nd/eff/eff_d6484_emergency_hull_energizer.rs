use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = ec::effects::EMERGENCY_HULL_ENERGIZER;
const A_EFFECT_ID: AEffectId = ac::effects::EMERGENCY_HULL_ENERGIZER;

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
        tracing::info!("effect {A_EFFECT_ID}: EHE effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.extend([
        make_hull_resist_mod(ac::attrs::HULL_EM_DMG_RESONANCE, ac::attrs::EM_DMG_RESONANCE),
        make_hull_resist_mod(ac::attrs::HULL_THERM_DMG_RESONANCE, ac::attrs::THERM_DMG_RESONANCE),
        make_hull_resist_mod(ac::attrs::HULL_KIN_DMG_RESONANCE, ac::attrs::KIN_DMG_RESONANCE),
        make_hull_resist_mod(ac::attrs::HULL_EXPL_DMG_RESONANCE, ac::attrs::EXPL_DMG_RESONANCE),
    ]);
}

fn make_hull_resist_mod(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id,
        op: AOp::PostMul,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id,
    }
}
