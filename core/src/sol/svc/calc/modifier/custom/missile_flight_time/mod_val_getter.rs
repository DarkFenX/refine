use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    sol::{
        AttrVal, ItemKey,
        svc::calc::{Calc, modifier::custom::shared::get_ship_key},
        uad::Uad,
    },
};

use super::{
    attr::{MISSILE_VELOCITY, SHIP_RADIUS},
    deps::reg_dependencies,
};

pub(in crate::sol::svc::calc::modifier) fn get_mod_val(
    calc: &mut Calc,
    uad: &Uad,
    item_key: ItemKey,
    a_effect_id: &ad::AEffectId,
) -> Option<AttrVal> {
    let ship_key = get_ship_key(uad, item_key)?;
    let missile_velocity = calc.get_item_attr_val_full(uad, item_key, &MISSILE_VELOCITY).ok()?;
    let ship_radius = calc.get_item_attr_val_full(uad, ship_key, &SHIP_RADIUS).ok()?;
    // Missile flight time is stored in milliseconds, thus have to multiply by 1000
    let val = ship_radius.dogma / missile_velocity.dogma * OF(1000.0);
    if val.is_infinite() {
        return None;
    }
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(calc, ship_key, item_key, *a_effect_id);
    Some(val)
}
