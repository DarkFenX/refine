use crate::sol::{FitId, ItemId, ItemTypeId, uad::item::Character};

pub struct CharacterInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub enabled: bool,
}
impl From<&Character> for CharacterInfo {
    fn from(sol_character: &Character) -> Self {
        CharacterInfo {
            id: sol_character.get_item_id(),
            type_id: sol_character.get_a_item_id(),
            fit_id: sol_character.get_fit_id(),
            enabled: sol_character.get_character_state(),
        }
    }
}
