use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct CharacterInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl CharacterInfo {
    fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&ssi::Character> for CharacterInfo {
    fn from(c: &ssi::Character) -> Self {
        CharacterInfo::new(c.id, c.fit_id, c.type_id, c.get_bool_state())
    }
}
