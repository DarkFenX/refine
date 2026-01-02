use crate::{
    api::{EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut},
    def::ItemTypeId,
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    fn api_get_mutator_id(&self, item_key: UItemId) -> ItemTypeId {
        self.u_data
            .items
            .get(item_key)
            .get_mutation_data()
            .unwrap()
            .get_mutator_id()
    }
}

impl<'a> Mutation<'a> {
    pub fn get_mutator_id(&self) -> ItemTypeId {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_mutator_id(),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.get_mutator_id(),
        }
    }
}

impl<'a> MutationMut<'a> {
    pub fn get_mutator_id(&self) -> ItemTypeId {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_mutator_id(),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.get_mutator_id(),
        }
    }
}

impl<'a> EffectiveMutation<'a> {
    pub fn get_mutator_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_id(self.item_key)
    }
}
impl<'a> EffectiveMutationMut<'a> {
    pub fn get_mutator_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_id(self.item_key)
    }
}

impl<'a> IncompleteMutation<'a> {
    pub fn get_mutator_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_id(self.item_key)
    }
}
impl<'a> IncompleteMutationMut<'a> {
    pub fn get_mutator_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_id(self.item_key)
    }
}
