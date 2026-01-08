use crate::{
    api::{EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn api_get_item_mutation(&self, item_uid: UItemId) -> Option<Mutation<'_>> {
        item_has_mutation_cache(self, item_uid).map(|v| match v {
            true => Mutation::Effective(EffectiveMutation::new(self, item_uid)),
            false => Mutation::Incomplete(IncompleteMutation::new(self, item_uid)),
        })
    }
    pub(in crate::api) fn api_get_item_mutation_mut(&mut self, item_uid: UItemId) -> Option<MutationMut<'_>> {
        item_has_mutation_cache(self, item_uid).map(|v| match v {
            true => MutationMut::Effective(EffectiveMutationMut::new(self, item_uid)),
            false => MutationMut::Incomplete(IncompleteMutationMut::new(self, item_uid)),
        })
    }
}

fn item_has_mutation_cache(sol: &SolarSystem, item_uid: UItemId) -> Option<bool> {
    sol.u_data
        .items
        .get(item_uid)
        .get_mutation_data()
        .map(|v| v.get_cache().is_some())
}
