// TODO: add anti-drone/fighter/npc TD and GD modifiers

use crate::{ac, ad};

pub(in crate::nd::eff) fn update_effect_td(a_effect_id: ad::AEffectId, a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: TD effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    add_td_modifiers(&mut a_effect.mods);
}

pub(in crate::nd::eff) fn update_effect_gd(a_effect_id: ad::AEffectId, a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: GD effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    add_gd_modifiers(&mut a_effect.mods);
}

pub(in crate::nd::eff) fn update_effect_wd(a_effect_id: ad::AEffectId, a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: WD effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    add_td_modifiers(&mut a_effect.mods);
    add_gd_modifiers(&mut a_effect.mods);
}

fn add_td_modifiers(mods: &mut Vec<ad::AEffectModifier>) {
    mods.reserve_exact(3);
    mods.push(make_td_mod(ac::attrs::MAX_RANGE_BONUS, ac::attrs::MAX_RANGE));
    mods.push(make_td_mod(ac::attrs::FALLOFF_BONUS, ac::attrs::FALLOFF));
    mods.push(make_td_mod(ac::attrs::TRACKING_SPEED_BONUS, ac::attrs::TRACKING_SPEED));
}

fn add_gd_modifiers(mods: &mut Vec<ad::AEffectModifier>) {
    mods.reserve_exact(4);
    mods.push(make_gd_mod(ac::attrs::MISSILE_VELOCITY_BONUS, ac::attrs::MAX_VELOCITY));
    mods.push(make_gd_mod(
        ac::attrs::EXPLOSION_DELAY_BONUS,
        ac::attrs::EXPLOSION_DELAY,
    ));
    mods.push(make_gd_mod(ac::attrs::AOE_CLOUD_SIZE_BONUS, ac::attrs::AOE_CLOUD_SIZE));
    mods.push(make_gd_mod(ac::attrs::AOE_VELOCITY_BONUS, ac::attrs::AOE_VELOCITY));
}

fn make_td_mod(affector_attr_id: ad::AAttrId, affectee_attr_id: ad::AAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::LocSrq(
            ad::AEffectLocation::Target,
            ad::AModifierSrq::ItemId(ac::items::GUNNERY),
        ),
        affectee_attr_id,
    }
}

fn make_gd_mod(affector_attr_id: ad::AAttrId, affectee_attr_id: ad::AAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::OwnSrq(ad::AModifierSrq::ItemId(
            ac::items::MISSILE_LAUNCHER_OPERATION,
        )),
        affectee_attr_id,
    }
}
