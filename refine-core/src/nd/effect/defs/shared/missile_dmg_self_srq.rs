use crate::ad::{
    AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectModStrength, AEffectModifier, AModifierSrq, AOp,
};

pub(in crate::nd::effect::defs) fn missile_dmg_self_srq_update_effect(
    effect_aid: AEffectId,
    a_effect: &mut AEffect,
    a_warnings: &mut Vec<String>,
    attr_aid: AAttrId,
) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {effect_aid}: self-skillreq missile dmg effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    let modifier = AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::DMG_MULT_BONUS),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::SelfRef),
        affectee_attr_id: attr_aid,
    };
    a_effect.modifiers.insert(modifier);
}
