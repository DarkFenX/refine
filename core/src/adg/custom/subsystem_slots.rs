// Subsystem amount attribute seems to have no effect on anything in EVE; the lib uses it to expose
// max amount of subsystem slots, si fix it from 5 in data to 4

use crate::{
    ad,
    defs::{EAttrId, EEffectId, OF},
    ec,
};

const SLOT_ATTR: EEffectId = ec::attrs::MAX_SUBSYSTEMS;
const SHIP_GROUP: EEffectId = ec::itemgrps::STRATEGIC_CRUISER;

pub(in crate::adg::custom) fn fix_subsysem_slot_amount(a_data: &mut ad::AData) {
    let mut applied = false;
    for item in a_data.items.iter_mut() {
        if item.grp_id != SHIP_GROUP {
            continue;
        }
        if let std::collections::hash_map::Entry::Occupied(mut entry) = item.attr_vals.entry(SLOT_ATTR) {
            if entry.insert(OF(4.0)) != OF(4.0) {
                applied = true;
            }
        }
    }
    if !applied {
        tracing::info!("fix for t3c subsystem amount attribute {SLOT_ATTR} wasn't applied");
    }
}
