use rc::{ItemCommon, Lender};

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, SideEffectInfo, SlotIndex,
    shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct BoosterInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<BoosterInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct BoosterInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub slot: Option<SlotIndex>,
    pub state: bool,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub side_effects: Vec<(EffectId, SideEffectInfo)>,
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
impl BoosterInfo {
    pub(in crate::info) fn from_core(
        core_booster: &mut rc::Booster,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let booster_id = core_booster.get_item_id();
        let booster_info_mode = item_info_modes.get(&booster_id);
        Self {
            id: booster_id,
            extended: match booster_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(BoosterInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Booster,
                    type_id: core_booster.get_type_id(),
                    fit_id: core_booster.get_fit_id(),
                    slot: core_booster.get_slot(),
                    state: core_booster.get_state(),
                    side_effects: core_booster
                        .iter_side_effects()
                        .map_into_iter(|core_side_effect| {
                            (
                                core_side_effect.get_effect_id(),
                                SideEffectInfo::from_core(core_side_effect),
                            )
                        })
                        .collect(),
                    effect_mode_overrides: core_booster.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_booster, booster_info_mode),
                }),
            },
        }
    }
}
