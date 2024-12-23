use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_drone_mutation(&mut self, item_id: &SolItemId) -> Result<(), RemoveDroneMutationError> {
        let item = self.items.get_item(item_id)?;
        self.svcs
            .unload_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
        let drone = match self.items.get_item_mut(item_id).unwrap().get_drone_mut() {
            Ok(drone) => drone,
            Err(error) => {
                let item = self.items.get_item(item_id).unwrap();
                self.svcs
                    .load_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
                return Err(error.into());
            }
        };
        if let Err(error) = drone.unmutate(&self.src) {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs
                .load_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
            return Err(error.into());
        }
        let item = self.items.get_item(item_id).unwrap();
        self.svcs
            .load_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
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
