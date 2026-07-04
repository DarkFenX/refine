use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
        AState,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_MICRO_WARP_DRIVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    // Make sure to apply self-modifiers even if fighter is disabled
    a_effect.state = AState::Disabled;
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: fighter MWD effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::FTR_ABIL_MWD_SPEED_BONUS),
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
            affectee_attr_id: AAttrId::MAX_VELOCITY,
        },
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::FTR_ABIL_MWD_SIG_RADIUS_BONUS),
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
            affectee_attr_id: AAttrId::SIG_RADIUS,
        },
    ]);
}
