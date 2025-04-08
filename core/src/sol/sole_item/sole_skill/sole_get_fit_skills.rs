use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::SkillInfo},
};

impl SolarSystem {
    pub fn get_fit_skills(&self, fit_id: &FitId) -> Result<Vec<SkillInfo>, GetFitSkillsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_skills_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_skills_internal(&self, fit_key: FitKey) -> Vec<SkillInfo> {
        self.uad
            .fits
            .get(fit_key)
            .skills
            .values()
            .map(|fit_skill| self.get_skill_internal(fit_skill.item_key).unwrap())
            .collect()
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
