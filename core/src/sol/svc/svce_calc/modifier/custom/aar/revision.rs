use crate::{ec, sol::item::SolItem};

pub(in crate::sol::svc::svce_calc::modifier) fn revise_on_item_add_removal(
    affector_item: &SolItem,
    changed_item: &SolItem,
) -> bool {
    match affector_item {
        SolItem::Module(module) => match module.get_charge_id() {
            Some(charge_id) => {
                changed_item.get_id() == charge_id && changed_item.get_type_id() == ec::items::NANITE_REPAIR_PASTE
            }
            // No charge on AAR -> not changing anything
            None => false,
        },
        // The modifier isn't supposed to be carried on anything but a module
        _ => false,
    }
}
