use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::ItemInfoMode;

pub struct CharacterInfo {
    pub id: rc::ItemId,
    pub extended: Option<CharacterInfoExt>,
}

pub struct CharacterInfoExt {
    #[cfg(feature = "serde")]
    kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub state: bool,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
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
                    kind: rc::ItemKind::Character,
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
