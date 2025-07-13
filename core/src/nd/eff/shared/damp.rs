use crate::{ac, ad};

pub(in crate::nd::eff) fn update_effect(a_effect_id: ad::AEffectId, a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: damp effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.reserve_exact(2);
    a_effect.mods.push(make_damp_mod(
        ac::attrs::MAX_TARGET_RANGE_BONUS,
        ac::attrs::MAX_TARGET_RANGE,
    ));
    a_effect.mods.push(make_damp_mod(
        ac::attrs::SCAN_RESOLUTION_BONUS,
        ac::attrs::SCAN_RESOLUTION,
    ));
}

fn make_damp_mod(affector_attr_id: ad::AAttrId, affectee_attr_id: ad::AAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
        affectee_attr_id,
    }
}
