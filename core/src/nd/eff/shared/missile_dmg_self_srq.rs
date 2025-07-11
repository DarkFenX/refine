use crate::{ac, ad};

pub(in crate::nd::eff) fn update_effect(
    a_effect_id: ad::AEffectId,
    a_effect: &mut ad::AEffect,
    a_attr_id: ad::AAttrId,
) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: self-skillreq missile dmg effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    let modifier = ad::AEffectModifier {
        affector_attr_id: ac::attrs::DMG_MULT_BONUS,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::OwnSrq(ad::AModifierSrq::SelfRef),
        affectee_attr_id: a_attr_id,
    };
    a_effect.mods.push(modifier);
    a_effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
}
