use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = ec::effects::ADAPTIVE_ARMOR_HARDENER;
const A_EFFECT_ID: AEffectId = ac::effects::ADAPTIVE_ARMOR_HARDENER;

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
        tracing::info!("effect {A_EFFECT_ID}: RAH effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect
        .mods
        .push(mk_rah_resonance_mod(ac::attrs::ARMOR_EM_DMG_RESONANCE));
    a_effect
        .mods
        .push(mk_rah_resonance_mod(ac::attrs::ARMOR_THERM_DMG_RESONANCE));
    a_effect
        .mods
        .push(mk_rah_resonance_mod(ac::attrs::ARMOR_KIN_DMG_RESONANCE));
    a_effect
        .mods
        .push(mk_rah_resonance_mod(ac::attrs::ARMOR_EXPL_DMG_RESONANCE));
}

fn mk_rah_resonance_mod(attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: attr_id,
        op: AOp::PreMul,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: attr_id,
    }
}
