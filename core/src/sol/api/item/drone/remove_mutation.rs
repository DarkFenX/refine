use crate::{
    err::basic::ItemMutatedError,
    sol::{ItemKey, SolarSystem, api::DroneMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_drone_mutation(&mut self, item_key: ItemKey) -> Result<(), ItemMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        let uad_drone = uad_item.get_drone().unwrap();
        if !uad_drone.has_mutation_data() {
            return Err(ItemMutatedError {
                item_id: uad_drone.get_item_id(),
            }
            .into());
        }
        self.svc.unload_item(&self.uad, item_key, uad_item);
        self.uad
            .items
            .get_mut(item_key)
            .get_drone_mut()
            .unwrap()
            .unmutate(&self.uad.src)
            .unwrap();
        let uad_item = self.uad.items.get(item_key);
        self.svc.load_item(&self.uad, item_key, uad_item);
        Ok(())
    }
}

impl<'a> DroneMut<'a> {
    pub fn unmutate(&mut self) -> Result<(), RemoveDroneMutationError> {
        self.sol.internal_remove_drone_mutation(self.key)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveDroneMutationError {
    #[error("{0}")]
    MutationNotSet(#[from] ItemMutatedError),
}
