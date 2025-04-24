use crate::{
    err::basic::ItemNotMutatedError,
    sol::{ItemKey, SolarSystem, api::DroneMut, uad::item::ItemAddMutation},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_drone_mutation(
        &mut self,
        item_key: ItemKey,
        mutation: ItemAddMutation,
    ) -> Result<(), ItemNotMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        self.svc.unload_item(&self.uad, item_key, uad_item);
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        if let Err(error) = uad_drone.mutate(&self.uad.src, mutation) {
            let item = self.uad.items.get(item_key);
            self.svc.load_item(&self.uad, item_key, item);
            return Err(error);
        }
        let uad_item = self.uad.items.get(item_key);
        self.svc.load_item(&self.uad, item_key, uad_item);
        Ok(())
    }
}

impl<'a> DroneMut<'a> {
    pub fn mutate(&mut self, mutation: ItemAddMutation) -> Result<(), AddDroneMutationError> {
        self.sol.internal_add_drone_mutation(self.key, mutation)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddDroneMutationError {
    #[error("{0}")]
    MutationAlreadySet(#[from] ItemNotMutatedError),
}
