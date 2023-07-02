use crate::{
    ad,
    defs::{EAttrId, EEffectId},
    ec,
    shr::{ModDomain, ModOp, State},
};

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    fix_online_effect_cat(a_data);
    mk_self_skillreq_modifiers_launcher_rof(a_data);
    mk_self_skillreq_modifier_missile_dmg(a_data, ec::effects::MISSILE_EM_DMG_BONUS, ec::attrs::EM_DMG);
    mk_self_skillreq_modifier_missile_dmg(a_data, ec::effects::MISSILE_THERM_DMG_BONUS, ec::attrs::THERM_DMG);
    mk_self_skillreq_modifier_missile_dmg(a_data, ec::effects::MISSILE_KIN_DMG_BONUS, ec::attrs::KIN_DMG);
    mk_self_skillreq_modifier_missile_dmg(a_data, ec::effects::MISSILE_EXPL_DMG_BONUS, ec::attrs::EXPL_DMG);
}

fn fix_online_effect_cat(a_data: &mut ad::AData) {
    let mut fixed = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == ec::effects::ONLINE) {
        if effect.state == State::Active {
            effect.state = State::Online;
            fixed = true;
        }
    }
    if !fixed {
        tracing::info!("\"online\" effect category did not need fixing")
    }
}

fn mk_self_skillreq_modifiers_launcher_rof(a_data: &mut ad::AData) {
    for effect in a_data.effects.iter_mut().filter(|v| v.id == ec::effects::SELF_ROF) {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq missile rof effect has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectAttrMod::new(
            ec::attrs::ROF_BONUS,
            ModOp::PostPerc,
            ad::AModTgtFilter::LocSrq(ModDomain::Ship, ad::AModSrq::SelfRef),
            ec::attrs::SPEED,
        );
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AModBuildStatus::Custom;
    }
}

fn mk_self_skillreq_modifier_missile_dmg(a_data: &mut ad::AData, effect_id: EEffectId, attr_id: EAttrId) {
    for effect in a_data.effects.iter_mut().filter(|v| v.id == effect_id) {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq missile dmg effect {effect_id} has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectAttrMod::new(
            ec::attrs::DMG_MULT_BONUS,
            ModOp::PostPerc,
            ad::AModTgtFilter::OwnSrq(ad::AModSrq::SelfRef),
            attr_id,
        );
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AModBuildStatus::Custom;
    }
}
