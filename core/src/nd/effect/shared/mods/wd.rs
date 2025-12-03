use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AModifierSrq, AOp},
};

pub(in crate::nd::effect) fn add_td_mods(a_effect_id: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: TD effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    add_td_modifiers(&mut a_effect.mods);
}

pub(in crate::nd::effect) fn add_gd_mods(a_effect_id: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: GD effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    add_gd_modifiers(&mut a_effect.mods);
}

pub(in crate::nd::effect) fn add_wd_mods(a_effect_id: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: WD effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    add_td_modifiers(&mut a_effect.mods);
    add_gd_modifiers(&mut a_effect.mods);
}

fn add_td_modifiers(mods: &mut Vec<AEffectModifier>) {
    mods.extend([
        // Modules
        make_td_loc_mod(ac::attrs::MAX_RANGE_BONUS, ac::attrs::MAX_RANGE),
        make_td_loc_mod(ac::attrs::FALLOFF_BONUS, ac::attrs::FALLOFF),
        make_td_loc_mod(ac::attrs::TRACKING_SPEED_BONUS, ac::attrs::TRACKING_SPEED),
        // Drones & NPCs
        make_direct_mod(ac::attrs::MAX_RANGE_BONUS, ac::attrs::MAX_RANGE),
        make_direct_mod(ac::attrs::FALLOFF_BONUS, ac::attrs::FALLOFF),
        make_direct_mod(ac::attrs::TRACKING_SPEED_BONUS, ac::attrs::TRACKING_SPEED),
        // Fighters - explosion radius is actually getting improved, which is a bug on CCP side
        make_direct_mod(
            ac::attrs::MAX_RANGE_BONUS,
            ac::attrs::FTR_ABIL_ATK_MISSILE_RANGE_OPTIMAL,
        ),
        make_direct_mod(ac::attrs::FALLOFF_BONUS, ac::attrs::FTR_ABIL_ATK_MISSILE_RANGE_FALLOFF),
        make_direct_mod(
            ac::attrs::TRACKING_SPEED_BONUS,
            ac::attrs::FTR_ABIL_ATK_MISSILE_EXPLOSION_RADIUS,
        ),
        make_direct_mod(
            ac::attrs::TRACKING_SPEED_BONUS,
            ac::attrs::FTR_ABIL_ATK_MISSILE_EXPLOSION_VELOCITY,
        ),
    ]);
}

fn add_gd_modifiers(mods: &mut Vec<AEffectModifier>) {
    mods.extend([
        // Modules
        make_gd_loc_mod(ac::attrs::MISSILE_VELOCITY_BONUS, ac::attrs::MAX_VELOCITY),
        make_gd_loc_mod(ac::attrs::EXPLOSION_DELAY_BONUS, ac::attrs::EXPLOSION_DELAY),
        make_gd_loc_mod(ac::attrs::AOE_CLOUD_SIZE_BONUS, ac::attrs::AOE_CLOUD_SIZE),
        make_gd_loc_mod(ac::attrs::AOE_VELOCITY_BONUS, ac::attrs::AOE_VELOCITY),
        // NPCs
        make_direct_mod(
            ac::attrs::MISSILE_VELOCITY_BONUS,
            ac::attrs::MISSILE_ENTITY_VELOCITY_MULTIPLIER,
        ),
        make_direct_mod(
            ac::attrs::EXPLOSION_DELAY_BONUS,
            ac::attrs::MISSILE_ENTITY_FLIGHT_TIME_MULTIPLIER,
        ),
        make_direct_mod(
            ac::attrs::AOE_CLOUD_SIZE_BONUS,
            ac::attrs::MISSILE_ENTITY_AOE_CLOUD_SIZE_MULTIPLIER,
        ),
        make_direct_mod(
            ac::attrs::AOE_VELOCITY_BONUS,
            ac::attrs::MISSILE_ENTITY_AOE_VELOCITY_MULTIPLIER,
        ),
        // Fighters
        make_direct_mod(ac::attrs::MISSILE_VELOCITY_BONUS, ac::attrs::FTR_ABIL_MISSILES_RANGE),
        make_direct_mod(ac::attrs::EXPLOSION_DELAY_BONUS, ac::attrs::FTR_ABIL_MISSILES_RANGE),
        make_direct_mod(
            ac::attrs::AOE_CLOUD_SIZE_BONUS,
            ac::attrs::FTR_ABIL_MISSILES_EXPLOSION_RADIUS,
        ),
        make_direct_mod(
            ac::attrs::AOE_VELOCITY_BONUS,
            ac::attrs::FTR_ABIL_MISSILES_EXPLOSION_VELOCITY,
        ),
    ]);
}

fn make_td_loc_mod(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            AEffectLocation::Target,
            AModifierSrq::TypeId(ac::items::GUNNERY),
        ),
        affectee_attr_id,
    }
}

fn make_gd_loc_mod(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            AEffectLocation::Target,
            AModifierSrq::TypeId(ac::items::MISSILE_LAUNCHER_OPERATION),
        ),
        affectee_attr_id,
    }
}

fn make_direct_mod(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id,
    }
}
