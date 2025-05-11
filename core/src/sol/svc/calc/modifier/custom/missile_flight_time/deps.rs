use super::attr::{MISSILE_FLIGHT_TIME, MISSILE_VELOCITY, SHIP_RADIUS};
use crate::{
    ad,
    sol::{ItemKey, svc::calc::Calc},
};

pub(super) fn reg_dependencies(
    calc: &mut Calc,
    ship_item_key: ItemKey,
    missile_item_key: ItemKey,
    missile_a_effect_id: ad::AEffectId,
) {
    calc.deps.add_with_source(
        missile_item_key,
        missile_a_effect_id,
        missile_item_key,
        MISSILE_VELOCITY,
        missile_item_key,
        MISSILE_FLIGHT_TIME,
    );
    calc.deps.add_with_source(
        missile_item_key,
        missile_a_effect_id,
        ship_item_key,
        SHIP_RADIUS,
        missile_item_key,
        MISSILE_FLIGHT_TIME,
    );
}
