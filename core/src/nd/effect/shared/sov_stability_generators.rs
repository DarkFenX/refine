use crate::{
    ad::{AEffectId, AItem, AItemEffectData, AItemId},
    util::RMap,
};

pub(in crate::nd::effect) fn assign_effect(
    a_items: &mut RMap<AItemId, AItem>,
    a_item_id: AItemId,
    a_effect_id: AEffectId,
) -> bool {
    match a_items.get_mut(&a_item_id) {
        Some(a_item) => {
            a_item.effect_datas.insert(a_effect_id, AItemEffectData::default());
            a_item.defeff_id = Some(a_effect_id);
            true
        }
        None => false,
    }
}
