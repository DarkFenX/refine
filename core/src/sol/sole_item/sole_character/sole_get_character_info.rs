use crate::{
    err::basic::ItemKindMatchError,
    sol::{ItemKey, SolarSystem, info::CharacterInfo},
};

impl SolarSystem {
    pub(in crate::sol) fn get_character_info_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<CharacterInfo, ItemKindMatchError> {
        let character = self.uad.items.get(item_key).get_character()?;
        Ok(CharacterInfo::from_character(&self.uad, character))
    }
}
