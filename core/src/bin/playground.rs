use reefast::consts::{EveEffectCategory, EveModDomain, EveModOperator};
use reefast::eve_type::{Attribute, Effect, Item, ItemModifier};
use std::collections::HashMap;

fn main() {
    let _attr = Attribute::new(
        0,
        Some(5),
        Some(50.0),
        false,
        false
    );
    let _eff = Effect::new(
        0,
        EveEffectCategory::Active,
        false,
        false,
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0)
    );
    let _mod = ItemModifier::new(
        EveModDomain::Ship,
        0,
        EveModOperator::PostPercent,
        0
    );
    let _item = Item::new (
        1,
        2,
        3,
        HashMap::new(),
        HashMap::new(),
        None
    );
}
