use crate::{
    defs::{ReeInt, SsFitId, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_character_info(&self, fit_id: &SsFitId) -> Result<ssn::SsCharacterInfo> {
        self.get_fit_character(fit_id).map(|v| v.into())
    }
    pub fn set_fit_character(
        &mut self,
        fit_id: SsFitId,
        a_item_id: ReeInt,
        state: bool,
    ) -> Result<ssn::SsCharacterInfo> {
        match self.remove_fit_character(&fit_id) {
            Ok(_) => (),
            // Suppress ItemTypeNotFound error, since this method is supposed to be used
            // even when no character is set
            Err(e) => match e.kind {
                ErrorKind::SsItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.items.alloc_item_id()?;
        let character = ssi::SsCharacter::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsCharacterInfo::from(&character);
        let item = ssi::SsItem::Character(character);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fit_character_state(&mut self, fit_id: &SsFitId, state: bool) -> Result<()> {
        self.get_fit_character_mut(fit_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_character(&mut self, fit_id: &SsFitId) -> Result<()> {
        let item_id = self.get_fit_character_id(fit_id)?;
        self.remove_item(&item_id)
    }
    // Non-public
    fn get_fit_character_id(&self, fit_id: &SsFitId) -> Result<SsItemId> {
        self.fits
            .get_fit(fit_id)?
            .character
            .ok_or_else(|| Error::new(ErrorKind::SsItemTypeNotFound(ssi::SsCharacter::get_name())))
    }
    fn get_fit_character(&self, fit_id: &SsFitId) -> Result<&ssi::SsCharacter> {
        let item_id = self.get_fit_character_id(fit_id)?;
        self.items.get_character(&item_id)
    }
    fn get_fit_character_mut(&mut self, fit_id: &SsFitId) -> Result<&mut ssi::SsCharacter> {
        let item_id = self.get_fit_character_id(fit_id)?;
        self.items.get_character_mut(&item_id)
    }
}
