use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolCharacter, SolItem},
        item_info::SolCharacterInfo,
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_character_info(&self, fit_id: &SolFitId) -> Result<SolCharacterInfo> {
        self.get_fit_character(fit_id).map(|v| v.into())
    }
    pub fn set_fit_character(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolCharacterInfo> {
        match self.remove_fit_character(&fit_id) {
            Ok(_) => (),
            // Suppress ItemTypeNotFound error, since this method is supposed to be used
            // even when no character is set
            Err(e) => match e.kind {
                ErrorKind::SolItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.items.alloc_item_id()?;
        let character = SolCharacter::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolCharacterInfo::from(&character);
        let item = SolItem::Character(character);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_character_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        self.items.get_character_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn set_fit_character_state(&mut self, fit_id: &SolFitId, state: bool) -> Result<()> {
        self.get_fit_character_mut(fit_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_character(&mut self, fit_id: &SolFitId) -> Result<()> {
        let item_id = self.get_fit_character_id(fit_id)?;
        self.remove_item(&item_id)
    }
    // Non-public
    fn get_fit_character_id(&self, fit_id: &SolFitId) -> Result<SolItemId> {
        self.fits
            .get_fit(fit_id)?
            .character
            .ok_or_else(|| Error::new(ErrorKind::SolItemTypeNotFound(SolCharacter::get_name())))
    }
    fn get_fit_character(&self, fit_id: &SolFitId) -> Result<&SolCharacter> {
        let item_id = self.get_fit_character_id(fit_id)?;
        self.items.get_character(&item_id)
    }
    fn get_fit_character_mut(&mut self, fit_id: &SolFitId) -> Result<&mut SolCharacter> {
        let item_id = self.get_fit_character_id(fit_id)?;
        self.items.get_character_mut(&item_id)
    }
}
