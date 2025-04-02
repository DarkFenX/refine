use crate::{
    ad,
    sol::{ItemId, svc::calc::Calc},
};

use super::attr::{MISSILE_FLIGHT_TIME, MISSILE_VELOCITY, SHIP_RADIUS};

pub(super) fn reg_dependencies(
    calc: &mut Calc,
    ship_item_id: ItemId,
    missile_item_id: ItemId,
    missile_a_effect_id: ad::AEffectId,
) {
    calc.deps.add_with_source(
        missile_item_id,
        missile_a_effect_id,
        missile_item_id,
        MISSILE_VELOCITY,
        missile_item_id,
        MISSILE_FLIGHT_TIME,
    );
    calc.deps.add_with_source(
        missile_item_id,
        missile_a_effect_id,
        ship_item_id,
        SHIP_RADIUS,
        missile_item_id,
        MISSILE_FLIGHT_TIME,
    );
}
