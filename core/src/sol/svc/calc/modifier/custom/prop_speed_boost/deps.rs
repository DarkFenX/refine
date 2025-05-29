use super::attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS, SHIP_SPEED};
use crate::sol::{
    ItemKey,
    svc::{AttrSpec, EffectSpec, calc::Calc},
};

pub(super) fn reg_dependencies(calc: &mut Calc, ship_item_key: ItemKey, prop_espec: EffectSpec) {
    let affectee_aspec = AttrSpec {
        item_key: ship_item_key,
        a_attr_id: SHIP_SPEED,
    };
    calc.deps.add_with_source(
        prop_espec,
        AttrSpec {
            item_key: prop_espec.item_key,
            a_attr_id: PROP_BOOST,
        },
        affectee_aspec,
    );
    calc.deps.add_with_source(
        prop_espec,
        AttrSpec {
            item_key: prop_espec.item_key,
            a_attr_id: PROP_THRUST,
        },
        affectee_aspec,
    );
    calc.deps.add_with_source(
        prop_espec,
        AttrSpec {
            item_key: ship_item_key,
            a_attr_id: SHIP_MASS,
        },
        affectee_aspec,
    );
}
