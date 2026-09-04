use rc::ItemCommon;

use crate::{EffectId, EffectMode, ItemInfoMode};

pub(in crate::info::item::info) fn get_effect_mode_overrides<T>(
    core_item: &T,
    item_info_mode: ItemInfoMode,
) -> Vec<(EffectId, EffectMode)>
where
    T: ItemCommon,
{
    match item_info_mode {
        ItemInfoMode::Id => Vec::new(),
        ItemInfoMode::Partial | ItemInfoMode::Full => core_item.iter_effect_mode_overrides().collect(),
    }
}
