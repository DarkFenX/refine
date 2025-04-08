use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, RmMode, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fit(&mut self, fit_id: &FitId) -> Result<(), RemoveFitError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        self.remove_fit_internal(fit_key);
        Ok(())
    }
    pub(in crate::sol) fn remove_fit_internal(&mut self, fit_key: FitKey) {
        let fit = self.uad.fits.get(fit_key);
        for item_key in fit.all_direct_items().into_iter() {
            self.remove_item_internal(item_key, RmMode::Free).unwrap();
        }
        self.svc.remove_fit(fit_key);
        let fit = self.uad.fits.remove(fit_key);
        if let Some(fleet_key) = fit.fleet {
            let fleet = self.uad.fleets.get_mut(fleet_key);
            fleet.remove_fit(&fit_key);
        }
    }
}

#[derive(Debug)]
pub enum RemoveFitError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for RemoveFitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for RemoveFitError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
