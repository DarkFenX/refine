use crate::{
    defs::{SkillLevel, SolItemId},
    ec,
    err::basic::{ItemFoundError, ItemKindMatchError, SkillLevelError},
    sol::{SolView, SolarSystem},
};

use super::check_skill_level;

impl SolarSystem {
    pub fn set_skill_level(&mut self, item_id: &SolItemId, level: SkillLevel) -> Result<(), SetSkillLevelError> {
        check_skill_level(level)?;
        self.items.get_item_mut(item_id)?.get_skill_mut()?.set_level(level);
        self.svcs.item_attr_value_changed(
            &SolView::new(
                &self.src,
                &self.fleets,
                &self.fits,
                &self.items,
                &self.default_incoming_dmg,
            ),
            item_id,
            &ec::attrs::SKILL_LEVEL,
        );
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetSkillLevelError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSkill(ItemKindMatchError),
    SkillLevelError(SkillLevelError),
}
impl std::error::Error for SetSkillLevelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSkill(e) => Some(e),
            Self::SkillLevelError(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetSkillLevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSkill(e) => e.fmt(f),
            Self::SkillLevelError(e) => e.fmt(f),
        }
    }
}
impl From<SkillLevelError> for SetSkillLevelError {
    fn from(error: SkillLevelError) -> Self {
        Self::SkillLevelError(error)
    }
}
impl From<ItemFoundError> for SetSkillLevelError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetSkillLevelError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSkill(error)
    }
}
