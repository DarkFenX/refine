use crate::{
    defs::{EEffectId, SolItemId},
    sol::svc::SolSvcs,
};

use super::attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS, SHIP_SPEED};

pub(super) fn reg_dependencies(
    svc: &mut SolSvcs,
    ship_item_id: SolItemId,
    prop_item_id: SolItemId,
    prop_effect_id: EEffectId,
) {
    svc.calc_data.deps.add_with_source(
        prop_item_id,
        prop_effect_id,
        prop_item_id,
        PROP_BOOST,
        ship_item_id,
        SHIP_SPEED,
    );
    svc.calc_data.deps.add_with_source(
        prop_item_id,
        prop_effect_id,
        prop_item_id,
        PROP_THRUST,
        ship_item_id,
        SHIP_SPEED,
    );
    svc.calc_data.deps.add_with_source(
        prop_item_id,
        prop_effect_id,
        ship_item_id,
        SHIP_MASS,
        ship_item_id,
        SHIP_SPEED,
    );
}
