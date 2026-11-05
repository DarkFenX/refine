use rc::ItemCommon;

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, shared::OvrdMapLight};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct StanceInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<StanceInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct StanceInfoExt {
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
impl StanceInfo {
    pub(in crate::info) fn from_core(
        core_stance: &mut rc::Stance,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let stance_id = core_stance.get_item_id();
        let stance_info_mode = item_info_modes.get(&stance_id);
        Self {
            id: stance_id,
            extended: match stance_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(StanceInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Stance,
                    type_id: core_stance.get_type_id(),
                    fit_id: core_stance.get_fit_id(),
                    state: core_stance.get_state(),
                    effect_mode_overrides: core_stance.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_stance, stance_info_mode),
                }),
            },
        }
    }
}
