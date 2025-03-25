use crate::{ac, sol::uad::item::Item};

pub(in crate::sol::svc::calc::modifier) fn revise_on_item_add_removal(
    affector_item: &Item,
    changed_item: &Item,
) -> bool {
    match affector_item {
        Item::Module(module) => match module.get_charge_item_id() {
            Some(charge_id) => {
                changed_item.get_item_id() == charge_id
                    && changed_item.get_a_item_id() == ac::items::NANITE_REPAIR_PASTE
            }
            // No charge on AAR -> not changing anything
            None => false,
        },
        // The modifier isn't supposed to be carried on anything but a module
        _ => false,
    }
}
