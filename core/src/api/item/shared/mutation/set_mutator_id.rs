use crate::{
    ad::AItemId,
    api::{EffectiveMutationMut, IncompleteMutationMut, ItemTypeId, MutationMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItem, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_mutator_id(
        &mut self,
        item_uid: UItemId,
        mutator_item_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        let u_item = self.u_data.items.get_mut(item_uid);
        match u_item {
            UItem::Drone(_) => self.internal_set_drone_mutator_id(item_uid, mutator_item_aid, reuse_eupdates),
            UItem::Module(_) => self.internal_set_module_mutator_aid(item_uid, mutator_item_aid, reuse_eupdates),
            _ => unreachable!("unmutable item kind is used to change mutator ID"),
        }
    }
}

impl<'a> MutationMut<'a> {
    pub fn set_mutator_id(self, mutator_id: ItemTypeId) -> MutationMut<'a> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.set_mutator_id(mutator_id),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.set_mutator_id(mutator_id),
        }
    }
}

impl<'a> EffectiveMutationMut<'a> {
    pub fn set_mutator_id(self, mutator_id: ItemTypeId) -> MutationMut<'a> {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_mutator_id(self.item_uid, mutator_id.into_aid(), &mut reuse_eupdates)
            .unwrap();
        self.sol.api_get_item_mutation_mut(self.item_uid).unwrap()
    }
}

impl<'a> IncompleteMutationMut<'a> {
    pub fn set_mutator_id(self, mutator_id: ItemTypeId) -> MutationMut<'a> {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_mutator_id(self.item_uid, mutator_id.into_aid(), &mut reuse_eupdates)
            .unwrap();
        self.sol.api_get_item_mutation_mut(self.item_uid).unwrap()
    }
}
