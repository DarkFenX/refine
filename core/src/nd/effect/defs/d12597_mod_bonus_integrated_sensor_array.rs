use crate::{
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::MOD_BONUS_INTEGRATED_SENSOR_ARRAY;
const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_INTEGRATED_SENSOR_ARRAY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Tested on 2026-04-06 on thunderdome, carrier can't cloak despite ISA having no modifiers to
    // transfer the cloak attribute
    let cloak_modifier = AEffectModifier {
        affector_attr_id: AAttrId::CAN_CLOAK,
        op: AOp::PostAssign,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::CAN_CLOAK,
    };
    for modifier in a_effect.modifiers.iter() {
        if modifier == &cloak_modifier {
            tracing::info!("effect {EFFECT_AID}: ISA effect already has cloak modifier");
            return;
        }
    }
    a_effect.modifiers.insert(cloak_modifier);
}
