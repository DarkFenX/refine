use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, EffectId, ItemAttrValues, ItemEffectInfo, ItemId, ItemInfoMode, ItemTypeId, Modification, ProjInfo,
    info::InfoModesInt,
};

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
impl ProjEffectInfo {
    pub(in crate::info) fn from_core(
        core_proj_effect: &mut rc::ProjEffectMut,
        item_info_modes: &InfoModesInt<ItemInfoMode, ItemId>,
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
                    projs: core_proj_effect.iter_projs().map(ProjInfo::from_core).collect(),
                    attrs: get_attrs(core_proj_effect, proj_effect_info_mode),
                    effects: get_effects(core_proj_effect, proj_effect_info_mode),
                    mods: get_mods(core_proj_effect, proj_effect_info_mode),
                }),
            },
        }
    }
}
