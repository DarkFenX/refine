use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
        AValue,
    },
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::INDUSTRIAL_COMPACT_CORE_EFFECT2;
const EFFECT_AID: AEffectId = AEffectId::INDUSTRIAL_COMPACT_CORE_EFFECT2;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Tested on 2026-04-07 on thunderdome, orca/porpoise can't cloak with industrial core running,
    // and there are no attributes and modifiers to transfer either of no-cloak attributes to ship
    a_effect.modifiers.insert(AEffectModifier {
        strength: AEffectModStrength::Hardcoded(AValue::from_f64(0.0)),
        op: AOp::PostAssign,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::CAN_CLOAK,
    });
}
