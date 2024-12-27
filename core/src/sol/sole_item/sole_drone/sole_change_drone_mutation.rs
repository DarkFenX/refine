use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::{item::SolItemChangeAttrMutation, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn change_drone_mutation(
        &mut self,
        item_id: &SolItemId,
        attr_mutations: Vec<SolItemChangeAttrMutation>,
    ) -> Result<(), ChangeDroneMutationError> {
        let drone = self.items.get_item_mut(item_id)?.get_drone_mut()?;
        let changed_attr_ids = drone.change_mutation_attrs(&self.src, attr_mutations)?;
        let sol_view = SolView::new(
            &self.src,
            &self.fleets,
            &self.fits,
            &self.items,
            &self.default_incoming_dmg,
        );
        for attr_id in changed_attr_ids {
            self.svcs.item_attr_value_changed(&sol_view, item_id, &attr_id);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ChangeDroneMutationError {
    ItemNotFound(ItemFoundError),
    ItemIsNotDrone(ItemKindMatchError),
    MutationNotSet(ItemMutatedError),
}
impl std::error::Error for ChangeDroneMutationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotDrone(e) => Some(e),
            Self::MutationNotSet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for ChangeDroneMutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotDrone(e) => e.fmt(f),
            Self::MutationNotSet(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for ChangeDroneMutationError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for ChangeDroneMutationError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotDrone(error)
    }
}
impl From<ItemMutatedError> for ChangeDroneMutationError {
    fn from(error: ItemMutatedError) -> Self {
        Self::MutationNotSet(error)
    }
}
