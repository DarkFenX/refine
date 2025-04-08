use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_proj_effect(&mut self, item_id: &ItemId) -> Result<(), RemoveProjEffectError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_proj_effect_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_proj_effect_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        // Check if everything is correct
        let item = self.uad.items.get(item_key);
        let proj_effect = item.get_proj_effect()?;
        // Remove outgoing projections
        for &projectee_item_key in proj_effect.get_projs().iter_projectee_item_keys() {
            // Update services
            let projectee_item = self.uad.items.get(projectee_item_key);
            self.svc
                .remove_item_projection(&self.uad, item_key, projectee_item_key, projectee_item);
            // Update user data - do not update info on projected effect, because projected effect
            // will be discarded anyway
            self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        }
        // Remove effect from services
        self.remove_item_key_from_svc(item_key);
        // Remove effect from user data
        self.uad.proj_effects.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveProjEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotProjEffect(ItemKindMatchError),
}
impl std::error::Error for RemoveProjEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotProjEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveProjEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotProjEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveProjEffectError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveProjEffectError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotProjEffect(error)
    }
}
