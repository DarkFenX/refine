use crate::sol::{
    FitId, ItemId, ItemTypeId,
    uad::{Uad, item::Character},
};

pub struct CharacterInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl CharacterInfo {
    pub(in crate::sol) fn from_character(uad: &Uad, character: &Character) -> Self {
        Self {
            id: character.get_item_id(),
            type_id: character.get_a_item_id(),
            fit_id: uad.fits.id_by_key(character.get_fit_key()),
            enabled: character.get_character_state(),
        }
    }
}
