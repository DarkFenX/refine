use crate::{ad, src::Src};

pub(super) fn get_side_effect_chance_attr_id(src: &Src, a_effect_id: &ad::AEffectId) -> Option<ad::AAttrId> {
    src.get_a_effect(a_effect_id)?.ae.chance_attr_id
}
