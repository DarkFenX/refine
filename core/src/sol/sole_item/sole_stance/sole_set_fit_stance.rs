use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::StanceInfo,
        uad::item::{Item, Stance},
    },
};

impl SolarSystem {
    pub fn set_fit_stance(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<StanceInfo, SetFitStanceError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.set_fit_stance_internal(fit_key, type_id, state);
        Ok(self.get_stance_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn set_fit_stance_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: bool,
    ) -> ItemKey {
        let fit = self.uad.fits.get(fit_key);
        // Remove old stance, if it was set
        if let Some(old_item_key) = fit.stance {
            self.remove_stance_internal(old_item_key).unwrap();
        }
        // Add new stance
        let item_id = self.uad.items.alloc_id();
        let stance = Stance::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = Item::Stance(stance);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.stance = Some(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(Debug)]
pub enum SetFitStanceError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for SetFitStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitStanceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
