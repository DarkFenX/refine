use crate::api::{Module, ModuleMut, Mutation, MutationMut};

impl<'a> Module<'a> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.uid)
    }
}

impl<'a> ModuleMut<'a> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.uid)
    }
    pub fn get_mutation_mut(&mut self) -> Option<MutationMut<'_>> {
        self.sol.api_get_item_mutation_mut(self.uid)
    }
}
