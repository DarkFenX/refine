use crate::ad::{
    AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AEffectModifiers, AItemId,
    AModifierSrq, AOp,
};

pub(in crate::nd::effect::data) fn add_td_mods(effect_aid: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {effect_aid}: TD effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    add_td_modifiers(&mut a_effect.modifiers);
}

pub(in crate::nd::effect::data) fn add_gd_mods(effect_aid: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {effect_aid}: GD effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    add_gd_modifiers(&mut a_effect.modifiers);
}

pub(in crate::nd::effect::data) fn add_wd_mods(effect_aid: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {effect_aid}: WD effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    add_td_modifiers(&mut a_effect.modifiers);
    add_gd_modifiers(&mut a_effect.modifiers);
}

fn add_td_modifiers(mods: &mut AEffectModifiers) {
    mods.extend([
        // Modules
        make_td_loc_mod(AAttrId::MAX_RANGE_BONUS, AAttrId::MAX_RANGE),
        make_td_loc_mod(AAttrId::FALLOFF_BONUS, AAttrId::FALLOFF),
        make_td_loc_mod(AAttrId::TRACKING_SPEED_BONUS, AAttrId::TRACKING_SPEED),
        // Drones & NPCs
        make_direct_mod(AAttrId::MAX_RANGE_BONUS, AAttrId::MAX_RANGE),
        make_direct_mod(AAttrId::FALLOFF_BONUS, AAttrId::FALLOFF),
        make_direct_mod(AAttrId::TRACKING_SPEED_BONUS, AAttrId::TRACKING_SPEED),
        // Fighters - explosion radius is actually getting improved, which is a bug on CCP side
        make_direct_mod(AAttrId::MAX_RANGE_BONUS, AAttrId::FTR_ABIL_ATK_MISSILE_RANGE_OPTIMAL),
        make_direct_mod(AAttrId::FALLOFF_BONUS, AAttrId::FTR_ABIL_ATK_MISSILE_RANGE_FALLOFF),
        make_direct_mod(
            AAttrId::TRACKING_SPEED_BONUS,
            AAttrId::FTR_ABIL_ATK_MISSILE_EXPLOSION_RADIUS,
        ),
        make_direct_mod(
            AAttrId::TRACKING_SPEED_BONUS,
            AAttrId::FTR_ABIL_ATK_MISSILE_EXPLOSION_VELOCITY,
        ),
    ]);
}

fn add_gd_modifiers(mods: &mut AEffectModifiers) {
    mods.extend([
        // Modules
        make_gd_loc_mod(AAttrId::MISSILE_VELOCITY_BONUS, AAttrId::MAX_VELOCITY),
        make_gd_loc_mod(AAttrId::EXPLOSION_DELAY_BONUS, AAttrId::EXPLOSION_DELAY),
        make_gd_loc_mod(AAttrId::AOE_CLOUD_SIZE_BONUS, AAttrId::AOE_CLOUD_SIZE),
        make_gd_loc_mod(AAttrId::AOE_VELOCITY_BONUS, AAttrId::AOE_VELOCITY),
        // NPCs
        make_direct_mod(
            AAttrId::MISSILE_VELOCITY_BONUS,
            AAttrId::MISSILE_ENTITY_VELOCITY_MULTIPLIER,
        ),
        make_direct_mod(
            AAttrId::EXPLOSION_DELAY_BONUS,
            AAttrId::MISSILE_ENTITY_FLIGHT_TIME_MULTIPLIER,
        ),
        make_direct_mod(
            AAttrId::AOE_CLOUD_SIZE_BONUS,
            AAttrId::MISSILE_ENTITY_AOE_CLOUD_SIZE_MULTIPLIER,
        ),
        make_direct_mod(
            AAttrId::AOE_VELOCITY_BONUS,
            AAttrId::MISSILE_ENTITY_AOE_VELOCITY_MULTIPLIER,
        ),
        // Fighters
        make_direct_mod(AAttrId::MISSILE_VELOCITY_BONUS, AAttrId::FTR_ABIL_MISSILES_RANGE),
        make_direct_mod(AAttrId::EXPLOSION_DELAY_BONUS, AAttrId::FTR_ABIL_MISSILES_RANGE),
        make_direct_mod(
            AAttrId::AOE_CLOUD_SIZE_BONUS,
            AAttrId::FTR_ABIL_MISSILES_EXPLOSION_RADIUS,
        ),
        make_direct_mod(
            AAttrId::AOE_VELOCITY_BONUS,
            AAttrId::FTR_ABIL_MISSILES_EXPLOSION_VELOCITY,
        ),
    ]);
}

fn make_td_loc_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: affector_attr_aid,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(AEffectLocation::Target, AModifierSrq::ItemId(AItemId::GUNNERY)),
        affectee_attr_id: affectee_attr_aid,
    }
}

fn make_gd_loc_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: affector_attr_aid,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            AEffectLocation::Target,
            AModifierSrq::ItemId(AItemId::MISSILE_LAUNCHER_OPERATION),
        ),
        affectee_attr_id: affectee_attr_aid,
    }
}

fn make_direct_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: affector_attr_aid,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: affectee_attr_aid,
    }
}
