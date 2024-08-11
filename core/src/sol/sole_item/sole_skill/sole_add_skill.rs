use crate::{
    defs::{EItemId, SkillLevel, SolFitId},
    err::basic::{FitFoundError, ItemAllocError, SkillLevelError},
    sol::{
        item::{SolItem, SolSkill},
        item_info::SolSkillInfo,
        SolarSystem,
    },
};

use super::check_skill_level;

impl SolarSystem {
    pub fn add_skill(
        &mut self,
        fit_id: SolFitId,
        a_item_id: EItemId,
        level: SkillLevel,
        state: bool,
    ) -> Result<SolSkillInfo, AddSkillError> {
        check_skill_level(level)?;
        let item_id = self.items.alloc_item_id()?;
        let skill = SolSkill::new(&self.src, item_id, fit_id, a_item_id, level, state);
        let info = SolSkillInfo::from(&skill);
        let item = SolItem::Skill(skill);
        let fit = self.fits.get_fit_mut(&fit_id)?;
        fit.skills.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddSkillError {
    InvalidSkillLevel(SkillLevelError),
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl std::error::Error for AddSkillError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidSkillLevel(e) => Some(e),
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddSkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidSkillLevel(e) => e.fmt(f),
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
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
impl From<ItemAllocError> for AddSkillError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
