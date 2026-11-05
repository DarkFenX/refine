use rc::ItemCommon;

use crate::{AttrId, EffectId, ItemAttrValues, ItemEffectInfo, ItemInfoMode, Modification};

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ItemInfoFull {
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, ItemAttrValues)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<(EffectId, ItemEffectInfo)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfoFull {
    pub(in crate::info::item::info) fn try_from_core<T>(core_item: &mut T, item_info_mode: ItemInfoMode) -> Option<Self>
    where
        T: ItemCommon,
    {
        match item_info_mode {
            ItemInfoMode::Id | ItemInfoMode::Partial => None,
            ItemInfoMode::Full => {
                // Separate statements, since each iterator borrows the item
                let attrs = match core_item.iter_attrs() {
                    Ok(attrs_iter) => attrs_iter.collect(),
                    Err(..) => Vec::new(),
                };
                let effects = match core_item.iter_effects() {
                    Ok(effects_iter) => effects_iter.collect(),
                    Err(..) => Vec::new(),
                };
                let mods = match core_item.iter_modifiers() {
                    Ok(v) => v.map(|(attr_id, attr_mods)| (attr_id, attr_mods.collect())).collect(),
                    Err(..) => Vec::new(),
                };
                Some(Self { attrs, effects, mods })
            }
        }
    }
}
