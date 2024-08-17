use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolCharacter,
};

pub struct SolCharacterInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub type_id: EItemId,
    pub enabled: bool,
}
impl SolCharacterInfo {
    fn new(id: SolItemId, fit_id: SolFitId, type_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&SolCharacter> for SolCharacterInfo {
    fn from(sol_character: &SolCharacter) -> Self {
        SolCharacterInfo::new(
            sol_character.get_id(),
            sol_character.get_fit_id(),
            sol_character.get_type_id(),
            sol_character.get_bool_state(),
        )
    }
}
