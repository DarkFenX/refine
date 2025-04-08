use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemMutatedError},
    sol::{ItemId, ItemKey, SolarSystem, uad::item::ItemChangeAttrMutation},
};

impl SolarSystem {
    pub fn change_drone_mutation(
        &mut self,
        item_id: &ItemId,
        attr_mutations: Vec<ItemChangeAttrMutation>,
    ) -> Result<(), ChangeDroneMutationError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.change_drone_mutation_internal(item_key, attr_mutations)
    }
    pub(in crate::sol) fn change_drone_mutation_internal(
        &mut self,
        item_key: ItemKey,
        attr_mutations: Vec<ItemChangeAttrMutation>,
    ) -> Result<(), ChangeDroneMutationError> {
        let drone = self.uad.items.get_mut(item_key).get_drone_mut()?;
        let changed_a_attr_ids = drone.change_mutation_attrs(&self.uad.src, attr_mutations)?;
        for a_attr_id in changed_a_attr_ids {
            self.svc.item_base_attr_value_changed(&self.uad, item_key, a_attr_id);
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
