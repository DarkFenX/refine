use crate::{
    api::{EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn api_get_item_mutation(&self, item_key: UItemId) -> Option<Mutation<'_>> {
        item_has_mutation_cache(self, item_key).map(|v| match v {
            true => Mutation::Effective(EffectiveMutation::new(self, item_key)),
            false => Mutation::Incomplete(IncompleteMutation::new(self, item_key)),
        })
    }
    pub(in crate::api) fn api_get_item_mutation_mut(&mut self, item_key: UItemId) -> Option<MutationMut<'_>> {
        item_has_mutation_cache(self, item_key).map(|v| match v {
            true => MutationMut::Effective(EffectiveMutationMut::new(self, item_key)),
            false => MutationMut::Incomplete(IncompleteMutationMut::new(self, item_key)),
        })
    }
}

fn item_has_mutation_cache(sol: &SolarSystem, item_key: UItemId) -> Option<bool> {
    sol.u_data
        .items
        .get(item_key)
        .get_mutation_data()
        .map(|v| v.get_cache().is_some())
}
