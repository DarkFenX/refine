use crate::{
    defs::{AttrVal, EEffectId, SolItemId, OF},
    sol::{svc::SolSvcs, SolView},
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    deps::reg_dependencies,
    misc::get_ship_id,
};

pub(in crate::sol::svc::svce_calc::modifier) fn get_mod_val(
    svc: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    effect_id: &EEffectId,
) -> Option<AttrVal> {
    let speed_boost = svc.calc_get_item_attr_val(sol_view, item_id, &PROP_BOOST).ok()?;
    let thrust = svc.calc_get_item_attr_val(sol_view, item_id, &PROP_THRUST).ok()?;
    let ship_id = get_ship_id(sol_view, item_id)?;
    let mass = svc.calc_get_item_attr_val(sol_view, &ship_id, &SHIP_MASS).ok()?;
    let perc = speed_boost.dogma * thrust.dogma / mass.dogma;
    if perc.is_infinite() {
        return None;
    }
    let val = OF(1.0) + perc / OF(100.0);
    // Register dependencies, so that affectee attribute is properly cleared up when any of affector
    // attributes change
    reg_dependencies(svc, ship_id, *item_id, *effect_id);
    Some(val)
}
