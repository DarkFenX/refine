use crate::{
    defs::SolItemId,
    sol::{svc::svce_calc::SolAffectorInfo, SolView},
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    misc::get_ship_id,
};

pub(in crate::sol::svc::svce_calc::modifier) fn get_affector_info(
    sol_view: &SolView,
    item_id: &SolItemId,
) -> Vec<SolAffectorInfo> {
    let mut affectors = Vec::new();
    if let Some(ship_id) = get_ship_id(sol_view, item_id) {
        affectors.push(SolAffectorInfo::new(*item_id, Some(PROP_BOOST)));
        affectors.push(SolAffectorInfo::new(*item_id, Some(PROP_THRUST)));
        affectors.push(SolAffectorInfo::new(ship_id, Some(SHIP_MASS)));
    }
    affectors
}
