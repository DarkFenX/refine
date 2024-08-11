use crate::{
    defs::{EItemId, SolFitId},
    err::basic::{FitFoundError, ItemAllocError},
    sol::{
        item::{SolBooster, SolItem},
        item_info::SolBoosterInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_booster(
        &mut self,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: bool,
    ) -> Result<SolBoosterInfo, AddBoosterError> {
        let item_id = self.items.alloc_item_id()?;
        let booster = SolBooster::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = self.make_booster_info(&booster);
        let item = SolItem::Booster(booster);
        let fit = self.fits.get_fit_mut(&fit_id)?;
        fit.boosters.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddBoosterError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl std::error::Error for AddBoosterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddBoosterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddBoosterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for AddBoosterError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
