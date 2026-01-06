use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::EMERGENCY_HULL_ENERGIZER;
const EFFECT_AID: AEffectId = AEffectId::EMERGENCY_HULL_ENERGIZER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        kills_item: true,
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: EHE effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        make_hull_resist_mod(AAttrId::HULL_EM_DMG_RESONANCE, AAttrId::EM_DMG_RESONANCE),
        make_hull_resist_mod(AAttrId::HULL_THERM_DMG_RESONANCE, AAttrId::THERM_DMG_RESONANCE),
        make_hull_resist_mod(AAttrId::HULL_KIN_DMG_RESONANCE, AAttrId::KIN_DMG_RESONANCE),
        make_hull_resist_mod(AAttrId::HULL_EXPL_DMG_RESONANCE, AAttrId::EXPL_DMG_RESONANCE),
    ]);
}

fn make_hull_resist_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: affector_attr_aid,
        op: AOp::PostMul,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: affectee_attr_aid,
    }
}
