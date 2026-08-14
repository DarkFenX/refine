use rc::ItemMutCommon;

use crate::{EffectId, ItemEffectInfo, ItemInfoMode, ItemInfoModes};

pub(in crate::info::item) fn get_effects<T>(core_item: &mut T, modes: ItemInfoModes) -> Vec<(EffectId, ItemEffectInfo)>
where
    T: ItemMutCommon,
{
    match modes.item {
        ItemInfoMode::Id | ItemInfoMode::Partial => Vec::new(),
        ItemInfoMode::Full => match core_item.iter_effects() {
            Ok(effects_iter) => effects_iter.collect(),
            Err(..) => Vec::new(),
        },
    }
}
