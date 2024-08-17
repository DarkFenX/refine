use crate::{
    defs::{AttrVal, SolItemId},
    ec,
    sol::{item::SolItem, svc::SolSvcs, SolView},
};

use super::affector_attr::AAR_AFFECTOR_ATTR_ID;

pub(in crate::sol::svc::svce_calc::modifier) fn get_mod_val(
    svc: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
) -> Option<AttrVal> {
    let item = sol_view.items.get_item(item_id).unwrap();
    match item {
        SolItem::Module(module) => {
            let charge_id = match module.get_charge_id() {
                Some(charge_id) => charge_id,
                // No charge - no extra reps
                None => return Some(1.0),
            };
            // If charge is referenced, we're supposed to always be able to fetch it
            let charge = sol_view.items.get_item(&charge_id).unwrap();
            if charge.get_type_id() == ec::items::NANITE_REPAIR_PASTE {
                match svc.calc_get_item_attr_val(sol_view, item_id, &AAR_AFFECTOR_ATTR_ID) {
                    Ok(sol_attr) => Some(sol_attr.dogma),
                    // Can't fetch multiplier attr - no extra reps
                    Err(_) => Some(1.0),
                }
            } else {
                // Different charge - no extra reps
                Some(1.0)
            }
        }
        // Not a module - don't calculate (should never happen with correct data)
        _ => None,
    }
}
