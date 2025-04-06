use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::SkillInfo},
};

impl SolarSystem {
    pub fn get_fit_skills(&self, fit_id: &FitId) -> Result<Vec<SkillInfo>, GetFitSkillsError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let skill_infos = fit
            .skills
            .values()
            .map(|v| SkillInfo::from(self.uad.items.get_by_id(&v.item_id).unwrap().get_skill().unwrap()))
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
