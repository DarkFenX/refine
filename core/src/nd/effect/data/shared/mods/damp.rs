use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
};

pub(in crate::nd::effect::data) fn add_damp_mods(a_effect_id: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: damp effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.extend([
        make_damp_mod(ac::attrs::MAX_TARGET_RANGE_BONUS, ac::attrs::MAX_TARGET_RANGE),
        make_damp_mod(ac::attrs::SCAN_RESOLUTION_BONUS, ac::attrs::SCAN_RESOLUTION),
    ]);
}

fn make_damp_mod(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id,
    }
}
