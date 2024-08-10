use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolCharacterInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_character_info(
        &self,
        fit_id: &SolFitId,
    ) -> Result<Option<SolCharacterInfo>, GetFitCharacterInfoError> {
        let fit = self.fits.get_fit(&fit_id)?;
        Ok(fit
            .character
            .map(|v| SolCharacterInfo::from(self.items.get_item(&v).unwrap().get_character().unwrap())))
    }
}

#[derive(Debug)]
pub enum GetFitCharacterInfoError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitCharacterInfoError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitCharacterInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitCharacterInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
