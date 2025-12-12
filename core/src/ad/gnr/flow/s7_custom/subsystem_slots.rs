// Subsystem count attribute seems to have no effect on anything in EVE; the lib uses it to expose
// max count of subsystem slots, so fix it from 5 in data to 4

use crate::{ac, ad, def::OF};

const SLOT_ATTR: ad::AAttrId = ac::attrs::MAX_SUBSYSTEMS;
const SHIP_GROUP: ad::AItemGrpId = ac::itemgrps::STRATEGIC_CRUISER;

pub(in crate::ad::gnr::flow::s7_custom) fn fix_subsysem_slot_count(a_data: &mut ad::AData) {
    let mut applied = false;
    for item in a_data.items.values_mut() {
        if item.grp_id != SHIP_GROUP {
            continue;
        }
        if let std::collections::hash_map::Entry::Occupied(mut entry) = item.attrs.entry(SLOT_ATTR)
            && entry.insert(OF(4.0)) != OF(4.0)
        {
            applied = true;
        }
    }
    if !applied {
        tracing::info!("attribute {SLOT_ATTR}: fix for t3c subsystem count wasn't applied");
    }
}
