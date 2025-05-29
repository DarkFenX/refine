use super::attr::{MISSILE_FLIGHT_TIME, MISSILE_VELOCITY, SHIP_RADIUS};
use crate::sol::{
    ItemKey,
    svc::{AttrSpec, EffectSpec, calc::Calc},
};

pub(super) fn reg_dependencies(calc: &mut Calc, ship_item_key: ItemKey, missile_espec: EffectSpec) {
    let affectee_aspec = AttrSpec {
        item_key: missile_espec.item_key,
        a_attr_id: MISSILE_FLIGHT_TIME,
    };
    calc.deps.add_with_source(
        missile_espec,
        AttrSpec {
            item_key: missile_espec.item_key,
            a_attr_id: MISSILE_VELOCITY,
        },
        affectee_aspec,
    );
    calc.deps.add_with_source(
        missile_espec,
        AttrSpec {
            item_key: ship_item_key,
            a_attr_id: SHIP_RADIUS,
        },
        affectee_aspec,
    );
}
