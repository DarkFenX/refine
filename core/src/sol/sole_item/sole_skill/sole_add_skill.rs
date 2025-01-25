use crate::{
    defs::{EItemId, SkillLevel, SolFitId},
    err::basic::{FitFoundError, SkillEveTypeError, SkillLevelError},
    sol::{
        info::SolSkillInfo,
        uad::{
            fit::SolFitSkill,
            item::{SolItem, SolSkill},
        },
        SolarSystem,
    },
};

use super::check_skill_level;

impl SolarSystem {
    pub fn add_skill(
        &mut self,
        fit_id: SolFitId,
        type_id: EItemId,
        level: SkillLevel,
        state: bool,
    ) -> Result<SolSkillInfo, AddSkillError> {
        check_skill_level(level)?;
        let item_id = self.uad.items.alloc_item_id();
        let skill = SolSkill::new(&self.uad.src, item_id, type_id, fit_id, level, state);
        let info = SolSkillInfo::from(&skill);
        let item = SolItem::Skill(skill);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        match fit.skills.entry(type_id) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(SolFitSkill::new(item_id, level));
            }
            std::collections::hash_map::Entry::Occupied(entry) => {
                return Err(SkillEveTypeError::new(type_id, fit_id, entry.get().item_id).into());
            }
        }
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddSkillError {
    InvalidSkillLevel(SkillLevelError),
    FitNotFound(FitFoundError),
    SkillIdCollision(SkillEveTypeError),
}
impl std::error::Error for AddSkillError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidSkillLevel(e) => Some(e),
            Self::FitNotFound(e) => Some(e),
            Self::SkillIdCollision(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddSkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidSkillLevel(e) => e.fmt(f),
            Self::FitNotFound(e) => e.fmt(f),
            Self::SkillIdCollision(e) => e.fmt(f),
        }
    }
}
impl From<SkillLevelError> for AddSkillError {
    fn from(error: SkillLevelError) -> Self {
        Self::InvalidSkillLevel(error)
    }
}
impl From<FitFoundError> for AddSkillError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<SkillEveTypeError> for AddSkillError {
    fn from(error: SkillEveTypeError) -> Self {
        Self::SkillIdCollision(error)
    }
}
