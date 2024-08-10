use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolFighterInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_fighter_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolFighterInfo>, GetFitFighterInfosError> {
        let fit = self.fits.get_fit(fit_id)?;
        let fighter_infos = fit
            .fighters
            .iter()
            .map(|v| self.make_fighter_info(self.items.get_item(v).unwrap().get_fighter().unwrap()))
            .collect();
        Ok(fighter_infos)
    }
}

#[derive(Debug)]
pub enum GetFitFighterInfosError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitFighterInfosError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitFighterInfosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitFighterInfosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
