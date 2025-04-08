use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_drone_mutation(&mut self, item_id: &ItemId) -> Result<(), RemoveDroneMutationError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.remove_drone_mutation_internal(item_key)
    }
    pub(in crate::sol) fn remove_drone_mutation_internal(
        &mut self,
        item_key: ItemKey,
    ) -> Result<(), RemoveDroneMutationError> {
        let item = self.uad.items.get(item_key);
        let drone = item.get_drone()?;
        if !drone.has_mutation_data() {
            return Err(ItemMutatedError {
                item_id: drone.get_item_id(),
            }
            .into());
        }
        self.svc.unload_item(&self.uad, item_key, item);
        self.uad
            .items
            .get_mut(item_key)
            .get_drone_mut()
            .unwrap()
            .unmutate(&self.uad.src)
            .unwrap();
        let item = self.uad.items.get(item_key);
        self.svc.load_item(&self.uad, item_key, item);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveDroneMutationError {
    ItemNotFound(ItemFoundError),
    ItemIsNotDrone(ItemKindMatchError),
    MutationNotSet(ItemMutatedError),
}
impl std::error::Error for RemoveDroneMutationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotDrone(e) => Some(e),
            Self::MutationNotSet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveDroneMutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotDrone(e) => e.fmt(f),
            Self::MutationNotSet(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveDroneMutationError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveDroneMutationError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotDrone(error)
    }
}
impl From<ItemMutatedError> for RemoveDroneMutationError {
    fn from(error: ItemMutatedError) -> Self {
        Self::MutationNotSet(error)
    }
}
