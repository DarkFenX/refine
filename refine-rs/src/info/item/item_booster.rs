use rc::{ItemCommon, Lender};

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification, SideEffectInfo,
    SlotIndex,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BoosterInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<BoosterInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
pub struct BoosterInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub slot: Option<SlotIndex>,
    pub state: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub side_effects: Vec<(EffectId, SideEffectInfo)>,
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
impl BoosterInfo {
    pub(in crate::info) fn from_core(core_booster: &mut rc::BoosterMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_booster.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(BoosterInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Booster,
                    type_id: core_booster.get_type_id(),
                    fit_id: core_booster.get_fit().get_fit_id(),
                    slot: core_booster.get_slot(),
                    state: core_booster.get_state(),
                    side_effects: core_booster
                        .iter_side_effects_mut()
                        .map_into_iter(|core_side_effect| {
                            (
                                core_side_effect.get_effect_id(),
                                SideEffectInfo::from_core(core_side_effect),
                            )
                        })
                        .collect(),
                    attrs: get_attrs(core_booster, item_mode),
                    effects: get_effects(core_booster, item_mode),
                    mods: get_mods(core_booster, item_mode),
                }),
            },
        }
    }
}
