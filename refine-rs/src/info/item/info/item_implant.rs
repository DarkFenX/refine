use rc::ItemCommon;

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, SlotIndex, shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ImplantInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<ImplantInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ImplantInfoExt {
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
impl ImplantInfo {
    pub(in crate::info) fn from_core(
        core_implant: &mut rc::Implant,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let implant_id = core_implant.get_item_id();
        let implant_info_mode = item_info_modes.get(&implant_id);
        Self {
            id: implant_id,
            extended: match implant_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ImplantInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Implant,
                    type_id: core_implant.get_type_id(),
                    fit_id: core_implant.get_fit_id(),
                    slot: core_implant.get_slot(),
                    state: core_implant.get_state(),
                    effect_mode_overrides: core_implant.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_implant, implant_info_mode),
                }),
            },
        }
    }
}
