use crate::sol::api::{Drone, DroneMut, Mutation, MutationMut};

impl<'a> Drone<'a> {
    pub fn get_mutation(&mut self) -> Option<Mutation> {
        self.sol.api_get_item_mutation(self.key)
    }
}

impl<'a> DroneMut<'a> {
    pub fn get_mutation(&mut self) -> Option<Mutation> {
        self.sol.api_get_item_mutation(self.key)
    }
    pub fn get_mutation_mut(&mut self) -> Option<MutationMut> {
        self.sol.api_get_item_mutation_mut(self.key)
    }
}
