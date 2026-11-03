use crate::api::{Module, Mutation};

impl<'s> Module<'s> {
    pub fn get_mutation(&mut self) -> Option<Mutation<'_>> {
        self.sol.api_get_item_mutation(self.uid)
    }
}
