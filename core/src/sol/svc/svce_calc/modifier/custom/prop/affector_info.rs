use crate::{
    defs::{EAttrId, SolItemId},
    sol::{svc::svce_calc::SolAffectorValueInfo, SolView},
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    misc::get_ship_id,
};

pub(in crate::sol::svc::svce_calc::modifier) fn get_affector_info(
    sol_view: &SolView,
    item_id: &SolItemId,
) -> Vec<(SolItemId, SolAffectorValueInfo)> {
    let mut affectors = Vec::new();
    if let Ok(Some(ship_id)) = get_ship_id(sol_view, item_id) {
        affectors.push((*item_id, SolAffectorValueInfo::AttrId(PROP_BOOST)));
        affectors.push((*item_id, SolAffectorValueInfo::AttrId(PROP_THRUST)));
        affectors.push((ship_id, SolAffectorValueInfo::AttrId(SHIP_MASS)));
    }
    affectors
}
