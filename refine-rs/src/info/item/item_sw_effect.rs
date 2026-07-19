use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, ItemId, ItemInfoMode, ItemTypeId, Modification};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SwEffectInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SwEffectInfoExt>,
}

#[cfg_attr(feature = "serde", serde_with::serde_as, derive(serde::Serialize))]
pub struct SwEffectInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub state: bool,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, AttrVals)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<(EffectId, EffectInfo)>,
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
impl SwEffectInfo {
    pub(in crate::info) fn from_core(core_sw_effect: &mut rc::SwEffectMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(SwEffectInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::SwEffect,
                    type_id: core_sw_effect.get_type_id(),
                    state: core_sw_effect.get_state(),
                    attrs: get_attrs(core_sw_effect, item_mode),
                    effects: get_effects(core_sw_effect, item_mode),
                    mods: get_mods(core_sw_effect, item_mode),
                }),
            },
        }
    }
}
