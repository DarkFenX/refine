use crate::sol::api::{Module, ModuleMut, Mutation, MutationMut};

impl<'a> Module<'a> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.key)
    }
}

impl<'a> ModuleMut<'a> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.key)
    }
    pub fn get_mutation_mut(&mut self) -> Option<MutationMut<'_>> {
        self.sol.api_get_item_mutation_mut(self.key)
    }
}
