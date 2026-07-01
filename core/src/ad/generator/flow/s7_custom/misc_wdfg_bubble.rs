// Allows WDFGs without scripts to use bubble effect

use crate::ad::{AAttrId, ADataGenerator, AEffectId, AItemAttr, AValue};

const BUBBLE_ATTR_ID: AAttrId = AAttrId::WARP_BUBBLE_STRENGTH;
const BUBBLE_EFFECT_ID: AEffectId = AEffectId::WARP_DISRUPT_SPHERE;

impl ADataGenerator {
    pub(super) fn add_wdfg_bubble_strength(&mut self) {
        let mut applied = false;
        for item in self.a_data.items.data.values_mut() {
            if !item.effects.contains_id(&BUBBLE_EFFECT_ID) {
                continue;
            }
            item.attrs.insert(AItemAttr {
                id: BUBBLE_ATTR_ID,
                value: AValue::from_f64(1.0),
            });
            applied = true;
        }
        if !applied {
            tracing::info!("attribute {BUBBLE_ATTR_ID}: WDFG bubble warp disruption strength fix wasn't applied");
        }
    }
}
