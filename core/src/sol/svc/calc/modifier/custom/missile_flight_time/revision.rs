use crate::sol::uad::item::Item;

pub(in crate::sol::svc::calc::modifier) fn revise_on_item_add_removal(
    affector_item: &Item,
    changed_item: &Item,
) -> bool {
    match changed_item {
        Item::Ship(changed_ship) => Some(changed_ship.get_fit_id()) == affector_item.get_fit_id(),
        _ => false,
    }
}
