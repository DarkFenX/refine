use smallvec::SmallVec;

use crate::sol::{
    ItemId,
    svc::calc::{AffectorInfo, modifier::custom::shared::get_ship_id},
    uad::Uad,
};

use super::attr::{MISSILE_VELOCITY, SHIP_RADIUS};

pub(in crate::sol::svc::calc::modifier) fn get_affector_info(uad: &Uad, item_id: &ItemId) -> SmallVec<AffectorInfo, 1> {
    let mut affectors = SmallVec::new();
    if let Some(ship_id) = get_ship_id(uad, item_id) {
        affectors.push(AffectorInfo {
            item_id: *item_id,
            attr_id: Some(MISSILE_VELOCITY),
        });
        affectors.push(AffectorInfo {
            item_id: ship_id,
            attr_id: Some(SHIP_RADIUS),
        });
    }
    affectors
}
