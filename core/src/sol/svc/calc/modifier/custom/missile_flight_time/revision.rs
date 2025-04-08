use crate::sol::{
    ItemKey,
    uad::{Uad, item::Item},
};

pub(in crate::sol::svc::calc::modifier) fn revise_on_item_add_removal(
    uad: &Uad,
    affector_key: ItemKey,
    changed_item: &Item,
) -> bool {
    match changed_item {
        Item::Ship(changed_ship) => Some(changed_ship.get_fit_key()) == uad.items.get(affector_key).get_fit_key(),
        _ => false,
    }
}
