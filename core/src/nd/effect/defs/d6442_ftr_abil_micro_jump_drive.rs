use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
        AState,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_MICRO_JUMP_DRIVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Make sure to apply self-modifiers even if fighter is disabled
    a_effect.state = AState::Disabled;
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: fighter MJD effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    // As of 2026-06-10, fighter sig blow during MJD does not work on TQ (tested by bombing
    // structure LR drones); reported it to the devs, not enabling it until it's fixed
    if false {
        a_effect.modifiers.insert(AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::FTR_ABIL_MJD_SIG_RADIUS_BONUS),
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
            affectee_attr_id: AAttrId::SIG_RADIUS,
        });
    }
}
