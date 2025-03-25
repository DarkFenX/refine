use smallvec::SmallVec;

use crate::sol::{ItemId, svc::calc::AffectorInfo, uad::Uad};

use super::{
    attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS},
    misc::get_ship_id,
};

pub(in crate::sol::svc::calc::modifier) fn get_affector_info(uad: &Uad, item_id: &ItemId) -> SmallVec<AffectorInfo, 1> {
    let mut affectors = SmallVec::new();
    if let Some(ship_id) = get_ship_id(uad, item_id) {
        affectors.push(AffectorInfo {
            item_id: *item_id,
            attr_id: Some(PROP_BOOST),
        });
        affectors.push(AffectorInfo {
            item_id: *item_id,
            attr_id: Some(PROP_THRUST),
        });
        affectors.push(AffectorInfo {
            item_id: ship_id,
            attr_id: Some(SHIP_MASS),
        });
    }
    affectors
}
