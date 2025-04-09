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

#[derive(thiserror::Error, Debug)]
pub enum ChangeDroneMutationError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotDrone(#[from] ItemKindMatchError),
    #[error("{0}")]
    MutationNotSet(#[from] ItemMutatedError),
}
