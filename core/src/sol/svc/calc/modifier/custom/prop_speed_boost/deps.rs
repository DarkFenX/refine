use super::attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS, SHIP_SPEED};
use crate::{
    ad,
    sol::{ItemKey, svc::calc::Calc},
};

pub(super) fn reg_dependencies(
    calc: &mut Calc,
    ship_item_key: ItemKey,
    prop_item_key: ItemKey,
    prop_a_effect_id: ad::AEffectId,
) {
    calc.deps.add_with_source(
        prop_item_key,
        prop_a_effect_id,
        prop_item_key,
        PROP_BOOST,
        ship_item_key,
        SHIP_SPEED,
    );
    calc.deps.add_with_source(
        prop_item_key,
        prop_a_effect_id,
        prop_item_key,
        PROP_THRUST,
        ship_item_key,
        SHIP_SPEED,
    );
    calc.deps.add_with_source(
        prop_item_key,
        prop_a_effect_id,
        ship_item_key,
        SHIP_MASS,
        ship_item_key,
        SHIP_SPEED,
    );
}
