use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::uad::item::SolCharacter,
};

pub struct SolCharacterInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub enabled: bool,
}
impl SolCharacterInfo {
    fn new(id: SolItemId, type_id: EItemId, fit_id: SolFitId, enabled: bool) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            enabled,
        }
    }
}
impl From<&SolCharacter> for SolCharacterInfo {
    fn from(sol_character: &SolCharacter) -> Self {
        SolCharacterInfo::new(
            sol_character.get_id(),
            sol_character.get_type_id(),
            sol_character.get_fit_id(),
            sol_character.get_character_state(),
        )
    }
}
