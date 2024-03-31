use crate::{
    ad,
    defs::{EAttrId, EEffectId},
    ec,
    shr::ModOp,
};

pub(in crate::adg::custom) fn mk_self_skillreq_modifier_missile_dmg(a_data: &mut ad::AData) {
    add_mod_for_effect_attr(a_data, ec::effects::MISSILE_EM_DMG_BONUS, ec::attrs::EM_DMG);
    add_mod_for_effect_attr(a_data, ec::effects::MISSILE_THERM_DMG_BONUS, ec::attrs::THERM_DMG);
    add_mod_for_effect_attr(a_data, ec::effects::MISSILE_KIN_DMG_BONUS, ec::attrs::KIN_DMG);
    add_mod_for_effect_attr(a_data, ec::effects::MISSILE_EXPL_DMG_BONUS, ec::attrs::EXPL_DMG);
}

fn add_mod_for_effect_attr(a_data: &mut ad::AData, effect_id: EEffectId, attr_id: EAttrId) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == effect_id) {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq missile dmg effect {effect_id} has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectAttrMod::new(
            ec::attrs::DMG_MULT_BONUS,
            ModOp::PostPerc,
            ad::AEffectTgtFilter::OwnSrq(ad::AModSrq::SelfRef),
            attr_id,
        );
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("self-skillreq missile dmg effect {effect_id} isn't found for customization");
    }
}
