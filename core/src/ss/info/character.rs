use crate::{
    defs::{ReeId, ReeInt},
    ss::item::Character,
};

pub struct CharacterInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl CharacterInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&Character> for CharacterInfo {
    fn from(c: &Character) -> Self {
        CharacterInfo::new(c.item_id, c.fit_id, c.type_id, c.get_bool_state())
    }
}
