use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolSkillInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_skill_info(&self, item_id: &SolItemId) -> Result<SolSkillInfo, GetSkillInfoError> {
        let skill = self.items.get_item(item_id)?.get_skill()?;
        Ok(SolSkillInfo::from(skill))
    }
}

#[derive(Debug)]
pub enum GetSkillInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSkill(ItemKindMatchError),
}
impl From<ItemFoundError> for GetSkillInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetSkillInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSkill(error)
    }
}
impl std::error::Error for GetSkillInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSkill(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetSkillInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSkill(e) => e.fmt(f),
        }
    }
}
