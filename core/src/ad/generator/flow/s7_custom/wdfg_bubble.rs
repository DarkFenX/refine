// Allows WDFGs without scripts to use bubble effect

use crate::{
    ac,
    ad::{AAttrId, AData, AEffectId, AValue},
};

const BUBBLE_ATTR: AAttrId = ac::attrs::DISALLOW_WARPING_JUMPING;
const BUBBLE_EFFECT: AEffectId = ac::effects::WARP_DISRUPT_SPHERE;

pub(in crate::ad::generator::flow::s7_custom) fn add_wdfg_bubble_strength(a_data: &mut AData) {
    let mut applied = false;
    for item in a_data.items.values_mut() {
        if !item.effect_datas.contains_key(&BUBBLE_EFFECT) {
            continue;
        }
        item.attrs.insert(BUBBLE_ATTR, AValue::new(1.0));
        applied = true;
    }
    if !applied {
        tracing::info!("attribute {BUBBLE_ATTR}: fix for t3c subsystem count wasn't applied");
    }
}
