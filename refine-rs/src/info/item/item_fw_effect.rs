use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FwEffectInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<FwEffectInfoExt>,
}

#[cfg_attr(feature = "serde", serde_with::serde_as, derive(serde::Serialize))]
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
impl FwEffectInfo {
    pub(in crate::info) fn from_core(core_fw_effect: &mut rc::FwEffectMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(FwEffectInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::FwEffect,
                    type_id: core_fw_effect.get_type_id(),
                    fit_id: core_fw_effect.get_fit().get_fit_id(),
                    state: core_fw_effect.get_state(),
                    attrs: get_attrs(core_fw_effect, item_mode),
                    effects: get_effects(core_fw_effect, item_mode),
                    mods: get_mods(core_fw_effect, item_mode),
                }),
            },
        }
    }
}
