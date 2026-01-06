use crate::{
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::FTR_ABIL_MJD;
const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_MJD;

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
        tracing::info!("effect {EFFECT_AID}: fighter MJD effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.push(AEffectModifier {
        affector_attr_id: AAttrId::FTR_ABIL_MJD_SIG_RADIUS_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
        affectee_attr_id: AAttrId::SIG_RADIUS,
    });
}
