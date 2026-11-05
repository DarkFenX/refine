use rc::ItemCommon;

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{EffectId, EffectMode, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, shared::OvrdMapLight};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SwEffectInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SwEffectInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct SwEffectInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
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
impl SwEffectInfo {
    pub(in crate::info) fn from_core(
        core_sw_effect: &mut rc::SwEffect,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let sw_effect_id = core_sw_effect.get_item_id();
        let sw_effect_info_mode = item_info_modes.get(&sw_effect_id);
        Self {
            id: sw_effect_id,
            extended: match sw_effect_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(SwEffectInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::SwEffect,
                    type_id: core_sw_effect.get_type_id(),
                    state: core_sw_effect.get_state(),
                    effect_mode_overrides: core_sw_effect.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_sw_effect, sw_effect_info_mode),
                }),
            },
        }
    }
}
