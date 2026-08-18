use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, EffectId, FitId, ItemAttrValues, ItemEffectInfo, ItemId, ItemInfoMode, ItemTypeId, Modification,
    shared::OverridableMap,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct FwEffectInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<FwEffectInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct FwEffectInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
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
impl FwEffectInfo {
    pub(in crate::info) fn from_core(
        core_fw_effect: &mut rc::FwEffectMut,
        item_info_modes: &OverridableMap<ItemId, ItemInfoMode>,
    ) -> Self {
        let fw_effect_id = core_fw_effect.get_item_id();
        let fw_effect_info_mode = item_info_modes.get(&fw_effect_id);
        Self {
            id: fw_effect_id,
            extended: match fw_effect_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(FwEffectInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::FwEffect,
                    type_id: core_fw_effect.get_type_id(),
                    fit_id: core_fw_effect.get_fit().get_fit_id(),
                    state: core_fw_effect.get_state(),
                    attrs: get_attrs(core_fw_effect, fw_effect_info_mode),
                    effects: get_effects(core_fw_effect, fw_effect_info_mode),
                    mods: get_mods(core_fw_effect, fw_effect_info_mode),
                }),
            },
        }
    }
}
