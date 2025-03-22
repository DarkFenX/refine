use crate::{ac, ad};

pub(in crate::adg::flow::custom) fn mk_self_skillreq_modifier_missile_dmg(a_data: &mut ad::AData) {
    add_mod_for_effect_attr(a_data, ac::effects::MISSILE_EM_DMG_BONUS, ac::attrs::EM_DMG);
    add_mod_for_effect_attr(a_data, ac::effects::MISSILE_THERM_DMG_BONUS, ac::attrs::THERM_DMG);
    add_mod_for_effect_attr(a_data, ac::effects::MISSILE_KIN_DMG_BONUS, ac::attrs::KIN_DMG);
    add_mod_for_effect_attr(a_data, ac::effects::MISSILE_EXPL_DMG_BONUS, ac::attrs::EXPL_DMG);
}

fn add_mod_for_effect_attr(a_data: &mut ad::AData, effect_id: ad::AEffectId, attr_id: ad::AAttrId) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == effect_id) {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq missile dmg effect {effect_id} has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectModifier {
            affector_attr_id: ac::attrs::DMG_MULT_BONUS,
            op: ad::AOp::PostPerc,
            affectee_filter: ad::AEffectAffecteeFilter::OwnSrq(ad::AModifierSrq::SelfRef),
            affectee_attr_id: attr_id,
        };
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("self-skillreq missile dmg effect {effect_id} is not found for customization");
    }
}
