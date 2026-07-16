use rc::ItemMutCommon;

use crate::{EffectId, EffectInfo, ItemInfoMode};

pub(in crate::info::item) fn get_effects<T>(core_item: &mut T, item_mode: ItemInfoMode) -> Vec<(EffectId, EffectInfo)>
where
    T: ItemMutCommon,
{
    match item_mode {
        ItemInfoMode::Id | ItemInfoMode::Partial => Vec::new(),
        ItemInfoMode::Full => match core_item.iter_effects() {
            Ok(effects_iter) => effects_iter.collect(),
            Err(_) => Vec::new(),
        },
    }
}
