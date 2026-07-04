use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
        AState,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_AFTERBURNER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, adg_warnings: &mut Vec<String>) {
    // Make sure to apply self-modifiers even if fighter is disabled
    a_effect.state = AState::Disabled;
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: fighter AB effect has modifiers, overwriting them");
        adg_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.insert(AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::FTR_ABIL_AB_SPEED_BONUS),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
        affectee_attr_id: AAttrId::MAX_VELOCITY,
    });
}
