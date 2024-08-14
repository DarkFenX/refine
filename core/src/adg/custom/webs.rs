use crate::{
    ad,
    defs::{EAttrId, EEffectId},
    ec,
};

pub(in crate::adg::custom) fn add_web_effect_modifiers(a_data: &mut ad::AData) {
    add_modifiers(a_data, ec::effects::REMOTE_WEBIFIER_FALLOFF);
    add_modifiers(a_data, ec::effects::STRUCTURE_MODULE_EFFECT_STASIS_WEBIFIER);
}

fn add_modifiers(a_data: &mut ad::AData, effect_id: EEffectId) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == effect_id) {
        if !effect.mods.is_empty() {
            tracing::info!("web effect {effect_id} has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectModifier::new(
            ec::attrs::SPEED_FACTOR,
            ad::AOp::PostPerc,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectDomain::Target),
            ec::attrs::MAX_VELOCITY,
        );
        effect.mods.push(modifier);
        applied = true;
    }
    if !applied {
        tracing::info!("web effect {effect_id} is not found for customization");
    }
}
