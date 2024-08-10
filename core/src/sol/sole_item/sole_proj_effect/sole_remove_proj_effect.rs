use itertools::Itertools;

use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_proj_effect(&mut self, item_id: &SolItemId) -> Result<(), RemoveProjEffectError> {
        // Check if everything is correct
        let item = self.items.get_item(item_id)?;
        let proj_effect = item.get_proj_effect()?;
        // Remove outgoing projections
        let proj_outgoing = proj_effect.projs.iter_items().map(|v| *v).collect_vec();
        for projectee_item_id in proj_outgoing {
            self.remove_proj_effect_proj(item_id, &projectee_item_id).unwrap();
        }
        // Remove effect from services
        let item = self.items.get_item(item_id).unwrap();
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        // Remove effect from skeleton
        self.proj_effects.remove(item_id);
        self.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveProjEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotProjEffect(ItemKindMatchError),
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
