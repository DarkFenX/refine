use crate::{
    ac,
    sol::{
        ItemKey,
        uad::{Uad, item::Item},
    },
};

pub(in crate::sol::svc::calc::modifier) fn revise_on_item_add_removal(
    uad: &Uad,
    affector_key: ItemKey,
    changed_item_key: ItemKey,
    changed_item: &Item,
) -> bool {
    match uad.items.get(affector_key) {
        Item::Module(module) => match module.get_charge_item_key() {
            Some(charge_key) => {
                changed_item_key == charge_key && changed_item.get_a_item_id() == ac::items::NANITE_REPAIR_PASTE
            }
            // No charge on AAR -> not changing anything
            None => false,
        },
        // The modifier isn't supposed to be carried on anything but a module
        _ => false,
    }
}
