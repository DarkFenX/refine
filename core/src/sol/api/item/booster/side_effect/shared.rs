use crate::{ad, src::Src};

pub(super) fn get_side_effect_chance_attr_id(src: &Src, a_effect_id: &ad::AEffectId) -> Option<ad::AAttrId> {
    let a_effect = match src.get_a_effect(a_effect_id) {
        Some(a_effect) => a_effect,
        None => return None,
    };
    a_effect.chance_attr_id
}
