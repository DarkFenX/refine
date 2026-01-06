use crate::{
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::ADAPTIVE_ARMOR_HARDENER;
const EFFECT_AID: AEffectId = AEffectId::ADAPTIVE_ARMOR_HARDENER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: RAH effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_rah_resonance_mod(AAttrId::ARMOR_EM_DMG_RESONANCE),
        mk_rah_resonance_mod(AAttrId::ARMOR_THERM_DMG_RESONANCE),
        mk_rah_resonance_mod(AAttrId::ARMOR_KIN_DMG_RESONANCE),
        mk_rah_resonance_mod(AAttrId::ARMOR_EXPL_DMG_RESONANCE),
    ]);
}

fn mk_rah_resonance_mod(attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: attr_id,
        op: AOp::PreMul,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: attr_id,
    }
}
