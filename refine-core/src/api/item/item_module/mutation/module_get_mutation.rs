use crate::api::{Module, ModuleMut, Mutation, MutationMut};

impl<'s> Module<'s> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.uid)
    }
}

impl<'s> ModuleMut<'s> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.uid)
    }
    pub fn get_mutation_mut(&mut self) -> Option<MutationMut<'_>> {
        self.sol.api_get_item_mutation_mut(self.uid)
    }
}
