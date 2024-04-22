use crate::{
    defs::SolItemId,
    sol::{svc::SolSvcs, SolView},
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS, SHIP_SPEED},
    misc::get_ship_id,
};

pub(super) fn reg_dependencies(svc: &mut SolSvcs, prop_id: SolItemId, ship_id: SolItemId) {
    svc.calc_data
        .deps
        .add_dependency(prop_id, PROP_BOOST, ship_id, SHIP_SPEED);
    svc.calc_data
        .deps
        .add_dependency(prop_id, PROP_THRUST, ship_id, SHIP_SPEED);
    svc.calc_data
        .deps
        .add_dependency(ship_id, SHIP_MASS, ship_id, SHIP_SPEED);
}

pub(in crate::sol::svc::svce_calc::modifier) fn on_effect_stop(
    svc: &mut SolSvcs,
    sol_view: &SolView,
    prop_id: &SolItemId,
) {
    // No dependencies if fit doesn't have a ship
    let ship_id = match get_ship_id(sol_view, prop_id) {
        Ok(ship_id_opt) => match ship_id_opt {
            Some(ship_id) => ship_id,
            None => return,
        },
        _ => return,
    };
    svc.calc_data
        .deps
        .remove_dependency(prop_id, &PROP_BOOST, &ship_id, &SHIP_SPEED);
    svc.calc_data
        .deps
        .remove_dependency(prop_id, &PROP_THRUST, &ship_id, &SHIP_SPEED);
    svc.calc_data
        .deps
        .remove_dependency(&ship_id, &SHIP_MASS, &ship_id, &SHIP_SPEED);
}
