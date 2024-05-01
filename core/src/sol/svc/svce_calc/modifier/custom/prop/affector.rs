use crate::{
    defs::{EAttrId, SolItemId},
    sol::SolView,
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    misc::get_ship_id,
};

pub(in crate::sol::svc::svce_calc::modifier) fn get_affectors(
    sol_view: &SolView,
    item_id: &SolItemId,
) -> Vec<(SolItemId, EAttrId)> {
    let mut affectors = Vec::new();
    if let Ok(Some(ship_id)) = get_ship_id(sol_view, item_id) {
        affectors.push((*item_id, PROP_BOOST));
        affectors.push((*item_id, PROP_THRUST));
        affectors.push((ship_id, SHIP_MASS));
    }
    affectors
}
