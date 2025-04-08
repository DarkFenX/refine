use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::SkillInfo},
};

impl SolarSystem {
    pub fn get_skill(&self, item_id: &ItemId) -> Result<SkillInfo, GetSkillError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_skill_internal(item_key)?)
    }
    pub(in crate::sol) fn get_skill_internal(&self, item_key: ItemKey) -> Result<SkillInfo, ItemKindMatchError> {
        let skill = self.uad.items.get(item_key).get_skill()?;
        Ok(SkillInfo::from_skill(&self.uad, skill))
    }
}

#[derive(Debug)]
pub enum GetSkillError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSkill(ItemKindMatchError),
}
impl std::error::Error for GetSkillError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSkill(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetSkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSkill(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetSkillError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetSkillError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSkill(error)
    }
}
