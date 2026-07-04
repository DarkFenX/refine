use crate::{
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectModStrength, AEffectModifier, AModifierSrq, AOp},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::DRONE_DMG_BONUS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, adg_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: self-skillreq drone dmg effect has modifiers, overwriting them");
        adg_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    let modifier = AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::DMG_MULT_BONUS),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::SelfRef),
        affectee_attr_id: AAttrId::DMG_MULT,
    };
    a_effect.modifiers.insert(modifier);
}
