use rc::ItemMutCommon;

use crate::{AttrId, ItemInfoMode, Modification};

pub(in crate::info::item) fn get_mods<T>(core_item: &mut T, item_mode: ItemInfoMode) -> Vec<(AttrId, Vec<Modification>)>
where
    T: ItemMutCommon,
{
    match item_mode {
        ItemInfoMode::Id | ItemInfoMode::Partial => Vec::new(),
        ItemInfoMode::Full => match core_item.iter_modifiers() {
            Ok(v) => v.map(|(attr_id, attr_mods)| (attr_id, attr_mods.collect())).collect(),
            Err(_) => Vec::new(),
        },
    }
}
