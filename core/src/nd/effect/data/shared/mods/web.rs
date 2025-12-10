use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
};

pub(in crate::nd::effect::data) fn add_web_mods(a_effect_id: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {a_effect_id}: web effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.push(AEffectModifier {
        affector_attr_id: ac::attrs::SPEED_FACTOR,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: ac::attrs::MAX_VELOCITY,
    });
}
