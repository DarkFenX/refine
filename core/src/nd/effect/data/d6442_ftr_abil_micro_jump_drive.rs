use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = ec::effects::FTR_ABIL_MJD;
const A_EFFECT_ID: AEffectId = ac::effects::FTR_ABIL_MJD;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: fighter MJD effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.push(AEffectModifier {
        affector_attr_id: ac::attrs::FTR_ABIL_MJD_SIG_RADIUS_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
        affectee_attr_id: ac::attrs::SIG_RADIUS,
    });
}
