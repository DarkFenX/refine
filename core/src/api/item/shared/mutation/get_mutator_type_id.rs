use crate::{
    api::{
        EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, ItemTypeId, Mutation,
        MutationMut,
    },
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    fn api_get_mutator_type_id(&self, item_uid: UItemId) -> ItemTypeId {
        ItemTypeId::from_aid(
            self.u_data
                .items
                .get(item_uid)
                .get_mutation_data()
                .unwrap()
                .get_mutator_type_aid(),
        )
    }
}

impl<'a> Mutation<'a> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_mutator_type_id(),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.get_mutator_type_id(),
        }
    }
}

impl<'a> MutationMut<'a> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_mutator_type_id(),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.get_mutator_type_id(),
        }
    }
}

impl<'a> EffectiveMutation<'a> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_type_id(self.item_uid)
    }
}
impl<'a> EffectiveMutationMut<'a> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_type_id(self.item_uid)
    }
}

impl<'a> IncompleteMutation<'a> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_type_id(self.item_uid)
    }
}
impl<'a> IncompleteMutationMut<'a> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_type_id(self.item_uid)
    }
}
