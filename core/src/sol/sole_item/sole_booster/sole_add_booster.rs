use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::BoosterInfo,
        uad::item::{Booster, Item},
    },
};

impl SolarSystem {
    pub fn add_booster(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<BoosterInfo, AddBoosterError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_booster_internal(fit_key, type_id, state);
        Ok(self.get_booster_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_booster_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: bool,
    ) -> ItemKey {
        let fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let booster = Booster::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = Item::Booster(booster);
        let item_key = self.uad.items.add(item);
        fit.boosters.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
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
