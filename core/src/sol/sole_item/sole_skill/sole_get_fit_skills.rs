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
            .map(|fit_skill| self.get_skill_info_internal(fit_skill.item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitSkillsError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
