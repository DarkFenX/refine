use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolCharacter,
};

pub struct SolCharacterInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SolCharacterInfo {
    fn new(id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SolCharacter> for SolCharacterInfo {
    fn from(sol_character: &SolCharacter) -> Self {
        SolCharacterInfo::new(
            sol_character.base.id,
            sol_character.fit_id,
            sol_character.base.a_item_id,
            sol_character.get_bool_state(),
        )
    }
}
