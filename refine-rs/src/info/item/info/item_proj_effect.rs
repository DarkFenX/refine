use rc::{ItemCommon, Lender};

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{EffectId, EffectMode, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, ProjInfo, shared::OvrdMapLight};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ProjEffectInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<ProjEffectInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ProjEffectInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub state: bool,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "proj_item_ids", skip_serializing_if = "Vec::is_empty")
    )]
    pub projs: Vec<ProjInfo>,
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
impl ProjEffectInfo {
    pub(in crate::info) fn from_core(
        core_proj_effect: &mut rc::ProjEffect,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let proj_effect_id = core_proj_effect.get_item_id();
        let proj_effect_info_mode = item_info_modes.get(&proj_effect_id);
        Self {
            id: proj_effect_id,
            extended: match proj_effect_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ProjEffectInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::ProjEffect,
                    type_id: core_proj_effect.get_type_id(),
                    state: core_proj_effect.get_state(),
                    projs: core_proj_effect
                        .iter_projs()
                        .map_into_iter(ProjInfo::from_core)
                        .collect(),
                    effect_mode_overrides: core_proj_effect.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_proj_effect, proj_effect_info_mode),
                }),
            },
        }
    }
}
