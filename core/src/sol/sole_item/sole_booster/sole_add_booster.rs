use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemId, ItemTypeId, SolarSystem,
        info::BoosterInfo,
        uad::item::{Booster, Item},
    },
};

impl SolarSystem {
    pub fn add_booster(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<BoosterInfo, AddBoosterError> {
        let item_id = self.add_booster_internal(fit_id, type_id, state)?;
        Ok(self.get_booster(&item_id).unwrap())
    }
    pub(in crate::sol) fn add_booster_internal(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<ItemId, AddBoosterError> {
        let item_id = self.uad.items.alloc_item_id();
        let booster = Booster::new(&self.uad.src, item_id, type_id, fit_id, state);
        let item = Item::Booster(booster);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.boosters.insert(item_id);
        self.uad.items.add(item);
        self.add_item_id_to_svc(&item_id);
        Ok(item_id)
    }
}

#[derive(Debug)]
pub enum AddBoosterError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddBoosterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddBoosterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddBoosterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
