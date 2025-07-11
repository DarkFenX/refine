use crate::{ad, util::RMap};

pub(in crate::nd::eff) fn assign_effect(
    a_items: &mut RMap<ad::AItemId, ad::AItem>,
    a_item_id: ad::AItemId,
    a_effect_id: ad::AEffectId,
) -> bool {
    match a_items.get_mut(&a_item_id) {
        Some(a_item) => {
            a_item.effect_datas.insert(a_effect_id, ad::AItemEffectData::default());
            a_item.defeff_id = Some(a_effect_id);
            true
        }
        None => false,
    }
}
