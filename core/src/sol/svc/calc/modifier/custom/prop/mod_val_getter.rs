use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    sol::{AttrVal, ItemId, svc::calc::Calc, uad::Uad},
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    deps::reg_dependencies,
    misc::get_ship_id,
};

pub(in crate::sol::svc::calc::modifier) fn get_mod_val(
    calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    a_effect_id: &ad::AEffectId,
) -> Option<AttrVal> {
    let speed_boost = calc.get_item_attr_val_full(uad, item_id, &PROP_BOOST).ok()?;
    let thrust = calc.get_item_attr_val_full(uad, item_id, &PROP_THRUST).ok()?;
    let ship_id = get_ship_id(uad, item_id)?;
    let mass = calc.get_item_attr_val_full(uad, &ship_id, &SHIP_MASS).ok()?;
    let perc = speed_boost.dogma * thrust.dogma / mass.dogma;
    if perc.is_infinite() {
        return None;
    }
    let val = OF(1.0) + perc / OF(100.0);
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(calc, ship_id, *item_id, *a_effect_id);
    Some(val)
}
