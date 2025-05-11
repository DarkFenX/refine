use smallvec::SmallVec;

use super::attr::{MISSILE_VELOCITY, SHIP_RADIUS};
use crate::sol::{
    ItemKey,
    svc::calc::{AffectorInfo, modifier::custom::shared::get_ship_key},
    uad::Uad,
};

pub(in crate::sol::svc::calc::modifier) fn get_affector_info(
    uad: &Uad,
    item_key: ItemKey,
) -> SmallVec<AffectorInfo, 1> {
    let mut affectors = SmallVec::new();
    if let Some(ship_key) = get_ship_key(uad, item_key) {
        affectors.push(AffectorInfo {
            item_id: uad.items.id_by_key(item_key),
            attr_id: Some(MISSILE_VELOCITY),
        });
        affectors.push(AffectorInfo {
            item_id: uad.items.id_by_key(ship_key),
            attr_id: Some(SHIP_RADIUS),
        });
    }
    affectors
}
