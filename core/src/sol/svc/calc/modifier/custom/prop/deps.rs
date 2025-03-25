use crate::{
    ad,
    sol::{ItemId, svc::calc::Calc},
};

use super::attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS, SHIP_SPEED};

pub(super) fn reg_dependencies(
    calc: &mut Calc,
    ship_item_id: ItemId,
    prop_item_id: ItemId,
    prop_a_effect_id: ad::AEffectId,
) {
    calc.deps.add_with_source(
        prop_item_id,
        prop_a_effect_id,
        prop_item_id,
        PROP_BOOST,
        ship_item_id,
        SHIP_SPEED,
    );
    calc.deps.add_with_source(
        prop_item_id,
        prop_a_effect_id,
        prop_item_id,
        PROP_THRUST,
        ship_item_id,
        SHIP_SPEED,
    );
    calc.deps.add_with_source(
        prop_item_id,
        prop_a_effect_id,
        ship_item_id,
        SHIP_MASS,
        ship_item_id,
        SHIP_SPEED,
    );
}
