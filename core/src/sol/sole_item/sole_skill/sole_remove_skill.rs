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

#[derive(thiserror::Error, Debug)]
pub enum RemoveSkillError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSkill(#[from] ItemKindMatchError),
}
