// In EVE, missiles are launched from center of an attacking ship. Ships have non-zero radius, and
// overview distance is calculated from surface to surface. To make missile range roughly match to
// their theoretical range, CCP added hidden flight time bonus, which depends on radius of the
// attacking ship. This effect implements it.

use crate::{
    ad::{AEffect, AEffectCatId, AEffectId, AItem, AItemEffect, AItemId, AState},
    nd::NEffect,
    svc::calc::CalcCustomModifier,
    util::RMap,
};

const EFFECT_AID: AEffectId = AEffectId::MISSILE_FLIGHT_TIME;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        calc_custom_mod: Some(CalcCustomModifier::MissileFlightTime),
        ..
    }
}

// ADG customizations
fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: AEffectCatId::PASSIVE,
        state: AState::Disabled,
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for item in a_items.values_mut().filter(|v| {
        v.effects.contains_id(&AEffectId::MISSILE_LAUNCHING)
            || v.effects.contains_id(&AEffectId::DEFENDER_MISSILE_LAUNCHING)
            || v.effects.contains_id(&AEffectId::FOF_MISSILE_LAUNCHING)
            || v.effects.contains_id(&AEffectId::DOT_MISSILE_LAUNCHING)
    }) {
        item.effects.insert(AItemEffect { id: EFFECT_AID, .. });
        assigned = true;
    }
    assigned
}
