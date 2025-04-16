use crate::{ad, src::Src};

pub(super) fn get_side_effect_chance_attr_id(src: &Src, a_effect_id: &ad::AEffectId) -> Option<ad::AAttrId> {
    let a_effect = match src.get_a_effect(a_effect_id) {
        Some(a_effect) => a_effect,
        None => return None,
    };
    let chance_attr_id = match a_effect.chance_attr_id {
        Some(chance_attr_id) => chance_attr_id,
        None => return None,
    };
    // Chance attribute is needed to calculate actual modified chance. No chance attribute - no
    // chance - not a side effect
    match src.get_a_attr(&chance_attr_id) {
        Some(_) => Some(chance_attr_id),
        None => None,
    }
}
