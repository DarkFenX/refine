use ordered_float::OrderedFloat as OF;

use crate::{
    ac,
    sol::{
        AttrVal, ItemId,
        svc::calc::Calc,
        uad::{Uad, item::Item},
    },
};

use super::affector_attr::AAR_MULTIPLIER;

pub(in crate::sol::svc::calc::modifier) fn get_mod_val(
    calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
) -> Option<AttrVal> {
    let item = uad.items.get_item(item_id).unwrap();
    match item {
        Item::Module(module) => {
            let charge_id = match module.get_charge_item_id() {
                Some(charge_id) => charge_id,
                // No charge - no extra reps
                None => return Some(OF(1.0)),
            };
            // If charge is referenced, we're supposed to always be able to fetch it
            let charge = uad.items.get_item(&charge_id).unwrap();
            if charge.get_a_item_id() == ac::items::NANITE_REPAIR_PASTE {
                match calc.get_item_attr_val_full(uad, item_id, &AAR_MULTIPLIER) {
                    Ok(sol_attr) => Some(sol_attr.dogma),
                    // Can't fetch multiplier attr - no extra reps
                    Err(_) => Some(OF(1.0)),
                }
            } else {
                // Different charge - no extra reps
                Some(OF(1.0))
            }
        }
        // Not a module - don't calculate (should never happen with correct data)
        _ => None,
    }
}
