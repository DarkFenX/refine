// Subsystem count attribute seems to have no effect on anything in EVE; the lib uses it to expose
// max count of subsystem slots, so fix it from 5 in data to 4

use std::collections::hash_map::Entry;

use crate::ad::{AAttrId, ADataGenerator, AItemGrpId, AValue};

const SLOT_ATTR: AAttrId = AAttrId::MAX_SUBSYSTEMS;
const SHIP_GROUP: AItemGrpId = AItemGrpId::STRATEGIC_CRUISER;

impl ADataGenerator {
    pub(super) fn fix_subsystem_slot_count(&mut self) {
        let mut applied = false;
        for item in self.a_data.items.data.values_mut() {
            if item.grp_id != SHIP_GROUP {
                continue;
            }
            let Entry::Occupied(mut entry) = item.attrs.entry(SLOT_ATTR) else {
                continue;
            };
            if entry.get().value != AValue::from_f64(4.0) {
                entry.get_mut().value = AValue::from_f64(4.0);
                applied = true;
            }
        }
        if !applied {
            let warning = format!("attribute {SLOT_ATTR}: fix for t3c subsystem count wasn't applied");
            self.a_data.warnings.customization.push(warning);
        }
    }
}
