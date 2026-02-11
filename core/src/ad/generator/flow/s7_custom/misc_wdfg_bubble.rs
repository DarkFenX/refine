// Allows WDFGs without scripts to use bubble effect

use crate::ad::{AAttrId, AData, AEffectId, AItemAttr, AValue};

const BUBBLE_ATTR_ID: AAttrId = AAttrId::DISALLOW_WARPING_JUMPING;
const BUBBLE_EFFECT_ID: AEffectId = AEffectId::WARP_DISRUPT_SPHERE;

pub(in crate::ad::generator::flow::s7_custom) fn add_wdfg_bubble_strength(a_data: &mut AData) {
    let mut applied = false;
    for item in a_data.items.data.values_mut() {
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
        tracing::info!("attribute {BUBBLE_ATTR_ID}: fix for t3c subsystem count wasn't applied");
    }
}
