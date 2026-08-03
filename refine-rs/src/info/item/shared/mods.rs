use rc::ItemMutCommon;

use crate::{ItemAttrModifications, ItemInfoMode};

pub(in crate::info::item) fn get_mods<T>(core_item: &mut T, item_mode: ItemInfoMode) -> Vec<ItemAttrModifications>
where
    T: ItemMutCommon,
{
    match item_mode {
        ItemInfoMode::Id | ItemInfoMode::Partial => Vec::new(),
        ItemInfoMode::Full => match core_item.iter_modifiers() {
            Ok(v) => v.collect(),
            Err(..) => Vec::new(),
        },
    }
}
