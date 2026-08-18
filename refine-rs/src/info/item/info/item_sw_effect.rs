use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, EffectId, ItemAttrValues, ItemEffectInfo, ItemId, ItemInfoMode, ItemTypeId, Modification,
    info::OverridableMap,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SwEffectInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SwEffectInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
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
impl SwEffectInfo {
    pub(in crate::info) fn from_core(
        core_sw_effect: &mut rc::SwEffectMut,
        item_info_modes: &OverridableMap<ItemId, ItemInfoMode>,
    ) -> Self {
        let sw_effect_id = core_sw_effect.get_item_id();
        let sw_effect_info_mode = item_info_modes.get(&sw_effect_id);
        Self {
            id: sw_effect_id,
            extended: match sw_effect_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(SwEffectInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::SwEffect,
                    type_id: core_sw_effect.get_type_id(),
                    state: core_sw_effect.get_state(),
                    attrs: get_attrs(core_sw_effect, sw_effect_info_mode),
                    effects: get_effects(core_sw_effect, sw_effect_info_mode),
                    mods: get_mods(core_sw_effect, sw_effect_info_mode),
                }),
            },
        }
    }
}
