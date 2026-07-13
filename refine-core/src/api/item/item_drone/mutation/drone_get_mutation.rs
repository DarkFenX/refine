use crate::api::{Drone, DroneMut, Mutation, MutationMut};

impl<'s> Drone<'s> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.uid)
    }
}

impl<'s> DroneMut<'s> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.uid)
    }
    pub fn get_mutation_mut(&mut self) -> Option<MutationMut<'_>> {
        self.sol.api_get_item_mutation_mut(self.uid)
    }
}
