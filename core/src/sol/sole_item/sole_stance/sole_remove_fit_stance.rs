use crate::{
    defs::SolFitId,
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{uad::item::SolStance, SolarSystem},
    util::Named,
};

impl SolarSystem {
    pub fn remove_fit_stance(&mut self, fit_id: &SolFitId) -> Result<(), RemoveFitStanceError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let item_id = match fit.stance {
            Some(item_id) => item_id,
            None => return Err(FitHasItemKindError::new(*fit_id, SolStance::get_name()).into()),
        };
        self.remove_stance(&item_id).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFitStanceError {
    FitNotFound(FitFoundError),
    FitHasNoStance(FitHasItemKindError),
}
impl std::error::Error for RemoveFitStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoStance(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFitStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoStance(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for RemoveFitStanceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitHasItemKindError> for RemoveFitStanceError {
    fn from(error: FitHasItemKindError) -> Self {
        Self::FitHasNoStance(error)
    }
}
