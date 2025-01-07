use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn remove_sw_effect(&mut self, item_id: &SolItemId) -> Result<(), RemoveSwEffectError> {
        let item = self.uad.items.get_item(item_id)?;
        // Just to check item kind
        item.get_sw_effect()?;
        self.svc.remove_item(&self.uad, item);
        self.uad.sw_effects.remove(item_id);
        self.uad.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveSwEffectError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSwEffect(ItemKindMatchError),
}
impl std::error::Error for RemoveSwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSwEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveSwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSwEffect(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveSwEffectError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveSwEffectError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSwEffect(error)
    }
}
