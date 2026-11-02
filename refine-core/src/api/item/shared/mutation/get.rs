use crate::{
    api::{DormantMutation, DormantMutationMut, EffectiveMutation, EffectiveMutationMut, Mutation, MutationMut},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api::item) fn api_get_item_mutation(&self, item_uid: UItemId) -> Option<Mutation<'_>> {
        is_item_mutation_effective(self, item_uid).map(|v| match v {
            true => Mutation::Effective(EffectiveMutation::new(self, item_uid)),
            false => Mutation::Dormant(DormantMutation::new(self, item_uid)),
        })
    }
    pub(in crate::api::item) fn api_get_item_mutation_mut(&mut self, item_uid: UItemId) -> Option<MutationMut<'_>> {
        is_item_mutation_effective(self, item_uid).map(|v| match v {
            true => MutationMut::Effective(EffectiveMutationMut::new(self, item_uid)),
            false => MutationMut::Dormant(DormantMutationMut::new(self, item_uid)),
        })
    }
}

fn is_item_mutation_effective(sol: &SolarSystem, item_uid: UItemId) -> Option<bool> {
    sol.u_data
        .items
        .get(item_uid)
        .get_mutation_data()
        .map(|v| v.is_effective())
}
