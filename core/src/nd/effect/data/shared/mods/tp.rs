use crate::ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp};

pub(in crate::nd::effect::data) fn add_tp_mods(effect_aid: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {effect_aid}: TP effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.push(AEffectModifier {
        affector_attr_id: AAttrId::SIG_RADIUS_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: AAttrId::SIG_RADIUS,
    });
}
