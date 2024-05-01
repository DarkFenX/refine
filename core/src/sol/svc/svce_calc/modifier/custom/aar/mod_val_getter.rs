use crate::{
    defs::{AttrVal, SolItemId},
    ec,
    sol::{
        item::{SolItem, SolModule},
        svc::SolSvcs,
        SolView,
    },
    util::{Error, ErrorKind, Named, Result},
};

use super::affector_attr::AAR_AFFECTOR_ATTR_ID;

pub(in crate::sol::svc::svce_calc::modifier) fn get_mod_val(
    svc: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
) -> Result<AttrVal> {
    let item = sol_view.items.get_item(item_id)?;
    match item {
        SolItem::Module(module) => {
            let charge_id = match module.charge_item_id {
                Some(charge_id) => charge_id,
                // No charge - no extra reps
                None => return Ok(1.0),
            };
            // Can't fetch charge - return error (since it should never happen)
            let charge = sol_view.items.get_item(&charge_id)?;
            if charge.get_a_item_id() == ec::items::NANITE_REPAIR_PASTE {
                match svc.calc_get_item_attr_val(sol_view, item_id, &AAR_AFFECTOR_ATTR_ID) {
                    Ok(sol_attr) => return Ok(sol_attr.dogma),
                    // Can't fetch multiplier attr - no extra reps
                    Err(_) => return Ok(1.0),
                }
            } else {
                // Different charge - no extra reps
                return Ok(1.0);
            }
        }
        // Not a module - return error (should never happen with correct data)
        _ => {
            return Err(Error::new(ErrorKind::UnexpectedItemType(
                item.get_id(),
                item.get_name(),
                SolModule::get_name(),
            )))
        }
    };
}
