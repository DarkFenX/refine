use rc::ItemMutCommon;

use crate::{AttrId, ItemAttrValues, ItemInfoMode};

pub(in crate::info::item) fn get_attrs<T>(
    core_item: &mut T,
    item_info_mode: ItemInfoMode,
) -> Vec<(AttrId, ItemAttrValues)>
where
    T: ItemMutCommon,
{
    match item_info_mode {
        ItemInfoMode::Id | ItemInfoMode::Partial => Vec::new(),
        ItemInfoMode::Full => match core_item.iter_attrs() {
            Ok(attrs_iter) => attrs_iter.collect(),
            Err(..) => Vec::new(),
        },
    }
}
