use crate::{
    api::{
        DormantMutation, DormantMutationMut, EffectiveMutation, EffectiveMutationMut, ItemTypeId, Mutation, MutationMut,
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

impl<'s> Mutation<'s> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_mutator_type_id(),
            Self::Dormant(dormant_mutation) => dormant_mutation.get_mutator_type_id(),
        }
    }
}

impl<'s> MutationMut<'s> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_mutator_type_id(),
            Self::Dormant(dormant_mutation) => dormant_mutation.get_mutator_type_id(),
        }
    }
}

impl<'s> EffectiveMutation<'s> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_type_id(self.item_uid)
    }
}
impl<'s> EffectiveMutationMut<'s> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_type_id(self.item_uid)
    }
}

impl<'s> DormantMutation<'s> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_type_id(self.item_uid)
    }
}
impl<'s> DormantMutationMut<'s> {
    pub fn get_mutator_type_id(&self) -> ItemTypeId {
        self.sol.api_get_mutator_type_id(self.item_uid)
    }
}
