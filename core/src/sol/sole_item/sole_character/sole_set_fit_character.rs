use crate::{
    defs::{EItemId, SolFitId},
    err::basic::{FitFoundError, ItemAllocError},
    sol::{
        item::{SolCharacter, SolItem},
        item_info::SolCharacterInfo,
        SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub fn set_fit_character(
        &mut self,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: bool,
    ) -> Result<SolCharacterInfo, SetFitCharacterError> {
        let fit = self.fits.get_fit(&fit_id)?;
        // Remove old character, if it was set
        if let Some(old_item_id) = fit.character {
            let old_item = self.items.get_item(&old_item_id).unwrap();
            self.svcs.remove_item(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                old_item,
            );
            self.items.remove_item(&old_item_id);
        }
        // Add new character
        // Should be fallible only if we didn't remove old character, so don't handle failure
        let item_id = self.items.alloc_item_id()?;
        let character = SolCharacter::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolCharacterInfo::from(&character);
        let item = SolItem::Character(character);
        let fit = self.fits.get_fit_mut(&fit_id).unwrap();
        fit.character = Some(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum SetFitCharacterError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl std::error::Error for SetFitCharacterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitCharacterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for SetFitCharacterError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
