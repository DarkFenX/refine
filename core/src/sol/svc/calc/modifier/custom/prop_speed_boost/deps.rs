use super::attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS, SHIP_SPEED};
use crate::sol::{
    ItemKey,
    svc::{EffectSpec, calc::Calc},
};

pub(super) fn reg_dependencies(calc: &mut Calc, ship_item_key: ItemKey, prop_espec: EffectSpec) {
    calc.deps.add_with_source(
        prop_espec.item_key,
        prop_espec.a_effect_id,
        prop_espec.item_key,
        PROP_BOOST,
        ship_item_key,
        SHIP_SPEED,
    );
    calc.deps.add_with_source(
        prop_espec.item_key,
        prop_espec.a_effect_id,
        prop_espec.item_key,
        PROP_THRUST,
        ship_item_key,
        SHIP_SPEED,
    );
    calc.deps.add_with_source(
        prop_espec.item_key,
        prop_espec.a_effect_id,
        ship_item_key,
        SHIP_MASS,
        ship_item_key,
        SHIP_SPEED,
    );
}
