use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    sol::{
        AttrVal, ItemId,
        svc::calc::{Calc, modifier::custom::shared::get_ship_id},
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
    item_id: &ItemId,
    a_effect_id: &ad::AEffectId,
) -> Option<AttrVal> {
    let ship_id = get_ship_id(uad, item_id)?;
    let missile_velocity = calc.get_item_attr_val_full(uad, item_id, &MISSILE_VELOCITY).ok()?;
    let ship_radius = calc.get_item_attr_val_full(uad, &ship_id, &SHIP_RADIUS).ok()?;
    // Missile flight time is stored in milliseconds, thus have to multiply by 1000
    let val = ship_radius.dogma / missile_velocity.dogma * OF(1000.0);
    if val.is_infinite() {
        return None;
    }
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(calc, ship_id, *item_id, *a_effect_id);
    Some(val)
}
