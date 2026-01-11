use crate::{
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectModifier, AModifierSrq, AOp},
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::DRONE_DMG_BONUS;
const EFFECT_AID: AEffectId = AEffectId::DRONE_DMG_BONUS;

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
        tracing::info!("effect {EFFECT_AID}: self-skillreq drone dmg effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    let modifier = AEffectModifier {
        affector_attr_id: AAttrId::DMG_MULT_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::SelfRef),
        affectee_attr_id: AAttrId::DMG_MULT,
    };
    a_effect.modifiers.insert(modifier);
}
