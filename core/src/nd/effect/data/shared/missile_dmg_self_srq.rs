use crate::ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectModifier, AModifierSrq, AOp};

pub(in crate::nd::effect::data) fn update_effect(effect_aid: AEffectId, a_effect: &mut AEffect, attr_aid: AAttrId) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {effect_aid}: self-skillreq missile dmg effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    let modifier = AEffectModifier {
        affector_attr_id: AAttrId::DMG_MULT_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::SelfRef),
        affectee_attr_id: attr_aid,
    };
    a_effect.modifiers.push(modifier);
}
