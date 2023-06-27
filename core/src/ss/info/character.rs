use crate::{
    defs::{ReeId, ReeInt},
    ss::item::SsCharacter,
};

pub struct SsCharacterInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub enabled: bool,
}
impl SsCharacterInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SsCharacter> for SsCharacterInfo {
    fn from(ss_character: &SsCharacter) -> Self {
        SsCharacterInfo::new(
            ss_character.id,
            ss_character.fit_id,
            ss_character.a_item_id,
            ss_character.get_bool_state(),
        )
    }
}
