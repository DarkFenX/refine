use rc::ItemCommon;

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, shared::OvrdMapLight};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct CharacterInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<CharacterInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct CharacterInfoExt {
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
impl CharacterInfo {
    pub(in crate::info) fn from_core(
        core_character: &mut rc::Character,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let character_id = core_character.get_item_id();
        let character_info_mode = item_info_modes.get(&character_id);
        Self {
            id: character_id,
            extended: match character_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(CharacterInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Character,
                    type_id: core_character.get_type_id(),
                    fit_id: core_character.get_fit_id(),
                    state: core_character.get_state(),
                    effect_mode_overrides: core_character.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_character, character_info_mode),
                }),
            },
        }
    }
}
