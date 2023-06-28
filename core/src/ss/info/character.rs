use crate::{
    defs::{ItemId, SsFitId, SsItemId},
    ss::item::SsCharacter,
};

pub struct SsCharacterInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: ItemId,
    pub enabled: bool,
}
impl SsCharacterInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: ItemId, enabled: bool) -> Self {
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
