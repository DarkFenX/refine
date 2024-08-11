use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{item_info::SolCharacterInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_character(&self, fit_id: &SolFitId) -> Result<Option<SolCharacterInfo>, GetFitCharacterError> {
        let fit = self.fits.get_fit(&fit_id)?;
        Ok(fit
            .character
            .map(|v| SolCharacterInfo::from(self.items.get_item(&v).unwrap().get_character().unwrap())))
    }
}

#[derive(Debug)]
pub enum GetFitCharacterError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitCharacterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitCharacterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
