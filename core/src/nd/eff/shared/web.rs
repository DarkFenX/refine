use crate::{ac, ad};

pub(in crate::nd::eff) fn update_effect(a_effect_id: ad::AEffectId, a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: web effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    let modifier = ad::AEffectModifier {
        affector_attr_id: ac::attrs::SPEED_FACTOR,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
        affectee_attr_id: ac::attrs::MAX_VELOCITY,
    };
    a_effect.mods.push(modifier);
}
