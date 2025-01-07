use crate::{
    defs::SolItemId,
    sol::{svc::calc::SolAffectorInfo, uad::SolUad},
};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    misc::get_ship_id,
};

pub(in crate::sol::svc::calc::modifier) fn get_affector_info(
    uad: &SolUad,
    item_id: &SolItemId,
) -> Vec<SolAffectorInfo> {
    let mut affectors = Vec::new();
    if let Some(ship_id) = get_ship_id(uad, item_id) {
        affectors.push(SolAffectorInfo::new(*item_id, Some(PROP_BOOST)));
        affectors.push(SolAffectorInfo::new(*item_id, Some(PROP_THRUST)));
        affectors.push(SolAffectorInfo::new(ship_id, Some(SHIP_MASS)));
    }
    affectors
}
