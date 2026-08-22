use rc::ItemMutCommon;

use crate::{EffectId, ItemEffectInfo, ItemInfoMode};

pub(in crate::info::item::info) fn get_effects<T>(
    core_item: &mut T,
    item_info_mode: ItemInfoMode,
) -> Vec<(EffectId, ItemEffectInfo)>
where
    T: ItemMutCommon,
{
    match item_info_mode {
        ItemInfoMode::Id | ItemInfoMode::Partial => Vec::new(),
        ItemInfoMode::Full => match core_item.iter_effects() {
            Ok(effects_iter) => effects_iter.collect(),
            Err(..) => Vec::new(),
        },
    }
}
