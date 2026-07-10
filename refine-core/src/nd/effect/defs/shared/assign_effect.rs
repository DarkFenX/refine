use crate::{
    ad::{AEffectId, AItem, AItemEffect, AItemId},
    util::RMap,
};

pub(in crate::nd::effect::defs) fn assign_defeff_to_item(
    a_items: &mut RMap<AItemId, AItem>,
    item_aid: AItemId,
    effect_aid: AEffectId,
) -> bool {
    match a_items.get_mut(&item_aid) {
        Some(a_item) => {
            a_item.effects.insert(AItemEffect { id: effect_aid, .. });
            a_item.defeff_id = Some(effect_aid);
            true
        }
        None => false,
    }
}

pub(in crate::nd::effect::defs) fn assign_to_item_with_eff(
    a_items: &mut RMap<AItemId, AItem>,
    check_effect_aid: AEffectId,
    assign_effect_aid: AEffectId,
) -> bool {
    let mut assigned = false;
    for a_item in a_items
        .values_mut()
        .filter(|v| v.effects.contains_id(&check_effect_aid))
    {
        a_item.effects.insert(AItemEffect {
            id: assign_effect_aid,
            ..
        });
        assigned = true;
    }
    assigned
}
