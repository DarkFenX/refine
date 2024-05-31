use crate::{ad, defs::EAttrId, ec::attrs, sol::item::SolItem};

// Fetches attribute ID which contains value defining how strong projectee will resist the effect
// projected to it.
pub(in crate::sol::svc::svce_calc) fn get_proj_effect_resist_attr_id(
    item: &SolItem,
    effect: &ad::AEffect,
) -> Option<EAttrId> {
    match effect.resist_attr_id {
        Some(resist_attr_id) => Some(resist_attr_id),
        _ => match item.get_a_item() {
            Ok(a_item) => a_item
                .attr_vals
                .get(&attrs::REMOTE_RESISTANCE_ID)
                .map(|v| *v as EAttrId),
            _ => None,
        },
    }
}
