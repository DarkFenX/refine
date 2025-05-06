use crate::{
    err::basic::ItemNotMutatedError,
    sol::{
        ItemAddMutation, ItemKey, ItemTypeId, SolarSystem,
        api::{AddMutationError, DroneMut, MutationMut},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_drone_mutation(
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
    pub fn mutate(&mut self, mutator_id: ItemTypeId) -> Result<MutationMut, AddMutationError> {
        let mutation = ItemAddMutation::new(mutator_id);
        self.sol.internal_add_drone_mutation(self.key, mutation)?;
        Ok(self.get_mutation_mut().unwrap())
    }
}
