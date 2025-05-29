use ordered_float::OrderedFloat as OF;

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    deps::reg_dependencies,
};
use crate::sol::{
    AttrVal,
    svc::{
        EffectSpec,
        calc::{Calc, modifier::custom::shared::get_ship_key},
    },
    uad::Uad,
};

pub(in crate::sol::svc::calc::modifier) fn get_mod_val(
    calc: &mut Calc,
    uad: &Uad,
    espec: EffectSpec,
) -> Option<AttrVal> {
    let ship_key = get_ship_key(uad, espec.item_key)?;
    let speed_boost = calc.get_item_attr_val_full(uad, espec.item_key, &PROP_BOOST).ok()?;
    let thrust = calc.get_item_attr_val_full(uad, espec.item_key, &PROP_THRUST).ok()?;
    let mass = calc.get_item_attr_val_full(uad, ship_key, &SHIP_MASS).ok()?;
    let perc = speed_boost.dogma * thrust.dogma / mass.dogma;
    if perc.is_infinite() {
        return None;
    }
    let val = OF(1.0) + perc / OF(100.0);
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(calc, ship_key, espec);
    Some(val)
}
