// Subsystem count attribute seems to have no effect on anything in EVE; the lib uses it to expose
// max count of subsystem slots, so fix it from 5 in data to 4

use crate::ad::{AAttrId, AData, AItemGrpId, AValue};

const SLOT_ATTR: AAttrId = AAttrId::MAX_SUBSYSTEMS;
const SHIP_GROUP: AItemGrpId = AItemGrpId::STRATEGIC_CRUISER;

pub(in crate::ad::generator::flow::s7_custom) fn fix_subsysem_slot_count(a_data: &mut AData) {
    let mut applied = false;
    for item in a_data.items.data.values_mut() {
        if item.grp_id != SHIP_GROUP {
            continue;
        }
        let mut entry = match item.attrs.entry(SLOT_ATTR) {
            std::collections::hash_map::Entry::Occupied(entry) => entry,
            _ => continue,
        };
        if entry.get().value != AValue::from_f64(4.0) {
            entry.get_mut().value = AValue::from_f64(4.0);
            applied = true;
        }
    }
    if !applied {
        tracing::info!("attribute {SLOT_ATTR}: fix for t3c subsystem count wasn't applied");
    }
}
