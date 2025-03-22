use crate::{ac, ad};

pub(in crate::adg::flow::custom) fn add_ship_web_modifiers(a_data: &mut ad::AData) {
    process_web_effect(a_data, ac::effects::REMOTE_WEBIFIER_FALLOFF);
}

pub(in crate::adg::flow::custom) fn add_structure_web_modifiers(a_data: &mut ad::AData) {
    process_web_effect(a_data, ac::effects::STRUCTURE_MODULE_EFFECT_STASIS_WEBIFIER);
}

pub(in crate::adg::flow::custom) fn add_drone_web_modifiers(a_data: &mut ad::AData) {
    process_web_effect(a_data, ac::effects::REMOTE_WEBIFIER_ENTITY);
}

fn process_web_effect(a_data: &mut ad::AData, effect_id: ad::AEffectId) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == effect_id) {
        if !effect.mods.is_empty() {
            tracing::info!("web effect {effect_id} has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectModifier {
            affector_attr_id: ac::attrs::SPEED_FACTOR,
            op: ad::AOp::PostPerc,
            affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
            affectee_attr_id: ac::attrs::MAX_VELOCITY,
        };
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("web effect {effect_id} is not found for customization");
    }
}
