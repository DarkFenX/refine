use crate::ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp};

pub(in crate::nd::effect::data) fn add_damp_mods(effect_aid: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {effect_aid}: damp effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        make_damp_mod(AAttrId::MAX_TARGET_RANGE_BONUS, AAttrId::MAX_TARGET_RANGE),
        make_damp_mod(AAttrId::SCAN_RESOLUTION_BONUS, AAttrId::SCAN_RESOLUTION),
    ]);
}

fn make_damp_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: affector_attr_aid,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: affectee_attr_aid,
    }
}
