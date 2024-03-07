use crate::{ec, ss::item::SsItem};

pub(in crate::ss::svc::svce_calc::modifier) fn revise_on_item_add_removal(
    src_item: &SsItem,
    changed_item: &SsItem,
) -> bool {
    match src_item {
        SsItem::Module(module) => match module.charge_item_id {
            Some(charge_item_id) => {
                changed_item.get_id() == charge_item_id
                    && changed_item.get_a_item_id() == ec::items::NANITE_REPAIR_PASTE
            }
            // No charge on AAR -> not changing anything
            None => false,
        },
        // The modifier isn't supposed to be carried on anything but a module
        _ => false,
    }
}
