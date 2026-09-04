use rc::ItemCommon;

use super::shared::{get_attrs, get_effect_mode_overrides, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, EffectId, EffectMode, FitId, ItemAttrValues, ItemEffectInfo, ItemId, ItemInfoMode, ItemTypeId,
    Modification, shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct AutochargeInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<AutochargeInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct AutochargeInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub cont_item_id: ItemId,
    pub cont_effect_id: EffectId,
    pub state: bool,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effect_mode_overrides: Vec<(EffectId, EffectMode)>,
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
impl AutochargeInfo {
    pub(super) fn from_core(
        core_autocharge: &mut rc::AutochargeMut,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let autocharge_id = core_autocharge.get_item_id();
        let autocharge_info_mode = item_info_modes.get(&autocharge_id);
        Self {
            id: autocharge_id,
            extended: match autocharge_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(AutochargeInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Autocharge,
                    type_id: core_autocharge.get_type_id(),
                    fit_id: core_autocharge.get_fit().get_fit_id(),
                    cont_item_id: core_autocharge.get_cont_item().get_item_id(),
                    cont_effect_id: core_autocharge.get_cont_effect_id(),
                    state: core_autocharge.get_state(),
                    effect_mode_overrides: get_effect_mode_overrides(core_autocharge, autocharge_info_mode),
                    attrs: get_attrs(core_autocharge, autocharge_info_mode),
                    effects: get_effects(core_autocharge, autocharge_info_mode),
                    mods: get_mods(core_autocharge, autocharge_info_mode),
                }),
            },
        }
    }
}
