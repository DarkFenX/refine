use rc::ItemCommon;

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, SlotIndex, shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SubsystemInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SubsystemInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct SubsystemInfoExt {
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
    pub effect_mode_overrides: Vec<(EffectId, EffectMode)>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub full: Option<ItemInfoFull>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SubsystemInfo {
    pub(in crate::info) fn from_core(
        core_subsystem: &mut rc::Subsystem,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let subsystem_id = core_subsystem.get_item_id();
        let subsystem_info_mode = item_info_modes.get(&subsystem_id);
        Self {
            id: subsystem_id,
            extended: match subsystem_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(SubsystemInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Subsystem,
                    type_id: core_subsystem.get_type_id(),
                    fit_id: core_subsystem.get_fit_id(),
                    slot: core_subsystem.get_slot(),
                    state: core_subsystem.get_state(),
                    effect_mode_overrides: core_subsystem.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_subsystem, subsystem_info_mode),
                }),
            },
        }
    }
}
