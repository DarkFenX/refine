use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectModifier, AModifierSrq, AOp},
};

pub(in crate::nd::eff) fn update_effect(a_effect_id: AEffectId, a_effect: &mut AEffect, a_attr_id: AAttrId) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: self-skillreq missile dmg effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    let modifier = AEffectModifier {
        affector_attr_id: ac::attrs::DMG_MULT_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::SelfRef),
        affectee_attr_id: a_attr_id,
    };
    a_effect.mods.push(modifier);
}
