use super::attr::{MISSILE_FLIGHT_TIME, MISSILE_VELOCITY, SHIP_RADIUS};
use crate::sol::{
    ItemKey,
    svc::{EffectSpec, calc::Calc},
};

pub(super) fn reg_dependencies(calc: &mut Calc, ship_item_key: ItemKey, missile_espec: EffectSpec) {
    calc.deps.add_with_source(
        missile_espec.item_key,
        missile_espec.a_effect_id,
        missile_espec.item_key,
        MISSILE_VELOCITY,
        missile_espec.item_key,
        MISSILE_FLIGHT_TIME,
    );
    calc.deps.add_with_source(
        missile_espec.item_key,
        missile_espec.a_effect_id,
        ship_item_key,
        SHIP_RADIUS,
        missile_espec.item_key,
        MISSILE_FLIGHT_TIME,
    );
}
