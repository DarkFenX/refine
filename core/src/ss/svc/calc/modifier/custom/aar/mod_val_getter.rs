use crate::{
    defs::{AttrVal, SsItemId},
    ec,
    ss::{
        item::{SsItem, SsModule},
        svc::SsSvcs,
        view::SsView,
    },
    util::{Error, ErrorKind, Named, Result},
};

use super::src_attr::AAR_SRC_ATTR_ID;

pub(in crate::ss::svc::calc::modifier) fn get_mod_val(
    svc: &mut SsSvcs,
    ss_view: &SsView,
    item_id: &SsItemId,
) -> Result<AttrVal> {
    let item = ss_view.items.get_item(item_id)?;
    match item {
        SsItem::Module(module) => {
            let charge_id = match module.charge_item_id {
                Some(charge_id) => charge_id,
                // No charge - no extra reps
                None => return Ok(1.0),
            };
            // Can't fetch charge - return error (since it should never happen)
            let charge = ss_view.items.get_item(&charge_id)?;
            if charge.get_a_item_id() == ec::items::NANITE_REPAIR_PASTE {
                match svc.calc_get_item_attr_val(ss_view, item_id, &AAR_SRC_ATTR_ID) {
                    Ok(ss_attr) => return Ok(ss_attr.dogma),
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
                SsModule::get_name(),
            )))
        }
    };
}
