use rc::ItemMutCommon;

use crate::info::ItemInfoMode;

pub(in crate::info::item) fn get_attrs<T>(core_item: &mut T, item_mode: ItemInfoMode) -> Vec<(rc::AttrId, rc::AttrVals)>
where
    T: ItemMutCommon,
{
    match item_mode {
        ItemInfoMode::Id | ItemInfoMode::Partial => Vec::new(),
        ItemInfoMode::Full => match core_item.iter_attrs() {
            Ok(attrs_iter) => attrs_iter.collect(),
            Err(_) => Vec::new(),
        },
    }
}
