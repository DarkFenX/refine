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

#[derive(thiserror::Error, Debug)]
pub enum AddBoosterError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
