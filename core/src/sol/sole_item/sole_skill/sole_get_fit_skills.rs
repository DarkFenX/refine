use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{item_info::SolSkillInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_skills(&self, fit_id: &SolFitId) -> Result<Vec<SolSkillInfo>, GetFitSkillsError> {
        let fit = self.fits.get_fit(fit_id)?;
        let skill_infos = fit
            .skills
            .iter()
            .map(|v| SolSkillInfo::from(self.items.get_item(v).unwrap().get_skill().unwrap()))
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
