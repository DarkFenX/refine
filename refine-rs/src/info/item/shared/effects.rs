use rc::ItemMutCommon;

use crate::{EffectId, ItemEffectInfo, ItemInfoArgs, ItemInfoMode};

pub(in crate::info::item) fn get_effects<T>(
    core_item: &mut T,
    info_args: ItemInfoArgs,
) -> Vec<(EffectId, ItemEffectInfo)>
where
    T: ItemMutCommon,
{
    match info_args.item {
        ItemInfoMode::Id | ItemInfoMode::Partial => Vec::new(),
        ItemInfoMode::Full => match core_item.iter_effects() {
            Ok(effects_iter) => effects_iter.collect(),
            Err(..) => Vec::new(),
        },
    }
}
