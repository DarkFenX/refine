use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{SolarSystem, info::SolSkillInfo},
};

impl SolarSystem {
    pub fn get_fit_skills(&self, fit_id: &SolFitId) -> Result<Vec<SolSkillInfo>, GetFitSkillsError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let skill_infos = fit
            .skills
            .values()
            .map(|v| SolSkillInfo::from(self.uad.items.get_item(&v.item_id).unwrap().get_skill().unwrap()))
            .collect();
        Ok(skill_infos)
    }
}

#[derive(Debug)]
pub enum GetFitSkillsError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitSkillsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitSkillsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitSkillsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
