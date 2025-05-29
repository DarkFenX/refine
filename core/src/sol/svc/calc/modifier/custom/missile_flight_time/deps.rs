use super::attr::{MISSILE_FLIGHT_TIME, MISSILE_VELOCITY, SHIP_RADIUS};
use crate::sol::{
    ItemKey,
    svc::{AttrSpec, EffectSpec, calc::Calc},
};

pub(super) fn reg_dependencies(calc: &mut Calc, ship_item_key: ItemKey, missile_espec: EffectSpec) {
    let affectee_aspec = AttrSpec::new(missile_espec.item_key, MISSILE_FLIGHT_TIME);
    calc.deps.add_with_source(
        missile_espec,
        AttrSpec::new(missile_espec.item_key, MISSILE_VELOCITY),
        affectee_aspec,
    );
    calc.deps
        .add_with_source(missile_espec, AttrSpec::new(ship_item_key, SHIP_RADIUS), affectee_aspec);
}
