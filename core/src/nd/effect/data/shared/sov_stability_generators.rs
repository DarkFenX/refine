use crate::{
    ad::{AEffectId, AItem, AItemEffectData, AItemId},
    util::RMap,
};

pub(in crate::nd::effect::data) fn assign_effect(
    a_items: &mut RMap<AItemId, AItem>,
    item_aid: AItemId,
    effect_aid: AEffectId,
) -> bool {
    match a_items.get_mut(&item_aid) {
        Some(a_item) => {
            a_item.effect_datas.insert(effect_aid, AItemEffectData::default());
            a_item.defeff_id = Some(effect_aid);
            true
        }
        None => false,
    }
}
