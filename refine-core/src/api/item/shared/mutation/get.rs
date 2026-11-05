use crate::{DormantMutation, EffectiveMutation, Mutation, sol::SolCore, ud::UItemId};

impl SolCore {
    pub(in crate::api::item) fn api_get_item_mutation(&mut self, item_uid: UItemId) -> Option<Mutation<'_>> {
        self.u_data
            .items
            .get(item_uid)
            .get_mutation_data()
            .map(|v| v.is_effective())
            .map(|v| match v {
                true => Mutation::Effective(EffectiveMutation::new(self, item_uid)),
                false => Mutation::Dormant(DormantMutation::new(self, item_uid)),
            })
    }
}
