use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification};

pub struct CharacterInfo {
    pub id: ItemId,
    pub extended: Option<CharacterInfoExt>,
}

pub struct CharacterInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: bool,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterInfo {
    pub(in crate::info) fn from_core(core_character: &mut rc::CharacterMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_character.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(CharacterInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Character,
                    type_id: core_character.get_type_id(),
                    fit_id: core_character.get_fit().get_fit_id(),
                    state: core_character.get_state(),
                    attrs: get_attrs(core_character, item_mode),
                    effects: get_effects(core_character, item_mode),
                    mods: get_mods(core_character, item_mode),
                }),
            },
        }
    }
}
