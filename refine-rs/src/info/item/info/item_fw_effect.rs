use rc::ItemCommon;

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, shared::OvrdMapLight};

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
    pub effect_mode_overrides: Vec<(EffectId, EffectMode)>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub full: Option<ItemInfoFull>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FwEffectInfo {
    pub(in crate::info) fn from_core(
        core_fw_effect: &mut rc::FwEffect,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
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
                    fit_id: core_fw_effect.get_fit_id(),
                    state: core_fw_effect.get_state(),
                    effect_mode_overrides: core_fw_effect.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_fw_effect, fw_effect_info_mode),
                }),
            },
        }
    }
}
