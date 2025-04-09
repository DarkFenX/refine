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

#[derive(thiserror::Error, Debug)]
pub enum GetSkillError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSkill(#[from] ItemKindMatchError),
}
