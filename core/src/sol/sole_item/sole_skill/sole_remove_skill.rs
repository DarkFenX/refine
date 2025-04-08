use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_skill(&mut self, item_id: &ItemId) -> Result<(), RemoveSkillError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_skill_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_skill_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let skill = item.get_skill()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(skill.get_fit_key());
        fit.skills.remove(&skill.get_a_item_id());
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveSkillError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSkill(ItemKindMatchError),
}
impl std::error::Error for RemoveSkillError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSkill(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveSkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSkill(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveSkillError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveSkillError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSkill(error)
    }
}
