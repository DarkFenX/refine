use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_proj_effect(&mut self, item_id: &SolItemId) -> Result<(), RemoveProjEffectError> {
        // Check if everything is correct
        let item = self.uad.items.get_item(item_id)?;
        let proj_effect = item.get_proj_effect()?;
        // Remove outgoing projections
        for projectee_item_id in proj_effect.get_projs().iter_items() {
            // Update services
            let projectee_item = self.uad.items.get_item(projectee_item_id).unwrap();
            self.svc.remove_item_projection(&self.uad, item, projectee_item);
            // Update user data - do not update info on projected effect, because projected effect
            // will be discarded anyway
            self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        }
        // Remove effect from services
        self.remove_item_id_from_svc(item_id);
        // Remove effect from user data
        self.uad.proj_effects.remove(item_id);
        self.uad.items.remove_item(item_id);
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
