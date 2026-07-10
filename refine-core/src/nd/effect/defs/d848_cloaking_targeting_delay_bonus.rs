use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier,
        AModifierSrq, AOp,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::CLOAKING_TARGETING_DELAY_BONUS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning =
            format!("effect {EFFECT_AID}: self-skillreq cloaking target delay effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    let modifier = AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::CLOAKING_TARGETING_DELAY_BONUS),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(AEffectLocation::Ship, AModifierSrq::SelfRef),
        affectee_attr_id: AAttrId::CLOAKING_TARGETING_DELAY,
    };
    a_effect.modifiers.insert(modifier);
}
