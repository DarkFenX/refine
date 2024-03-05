use crate::{
    defs::{AttrVal, SsItemId},
    ec,
    ss::{item::SsItem, svc::SsSvcs, view::SsView},
    util::Result,
};

pub(super) fn get_mod_val(svc: &mut SsSvcs, ss_view: &SsView, item_id: &SsItemId) -> Result<AttrVal> {
    let item = ss_view.items.get_item(item_id)?;
    match item {
        SsItem::Module(module) => {
            let charge_id = match module.charge_ss_item_id {
                Some(charge_id) => charge_id,
                // No charge - no extra reps
                None => return Ok(1.0),
            };
            let charge = match ss_view.items.get_item(&charge_id) {
                Ok(charge) => charge,
                // Can't fetch charge data - no extra reps
                // Shouldn't ever happen, added just for safety
                Err(_) => return Ok(1.0),
            };
            if charge.get_a_item_id() == ec::items::NANITE_REPAIR_PASTE {
                match svc.calc_get_item_attr_val(ss_view, item_id, &ec::attrs::CHARGED_ARMOR_DMG_MULT) {
                    Ok(ss_attr) => return Ok(ss_attr.dogma),
                    // Can't fetch multiplier attr - no extra reps
                    Err(_) => return Ok(1.0),
                }
            } else {
                // Different charge - no extra reps
                return Ok(1.0);
            }
        }
        // Not a module - no extra reps
        _ => return Ok(1.0),
    };
}
