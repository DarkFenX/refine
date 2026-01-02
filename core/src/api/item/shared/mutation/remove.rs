use crate::{
    api::{EffectiveMutationMut, IncompleteMutationMut, MutationMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItem, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_item_mutation(
        &mut self,
        item_key: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        match self.u_data.items.get(item_key) {
            UItem::Drone(_) => self.internal_remove_drone_mutation(item_key, reuse_eupdates),
            UItem::Module(_) => self.internal_remove_module_mutation(item_key, reuse_eupdates),
            _ => unreachable!("unmutable item kind is used in mutation"),
        }
    }
}

impl<'a> MutationMut<'a> {
    pub fn remove(self) {
        match self {
            MutationMut::Effective(effective_mutation) => effective_mutation.remove(),
            MutationMut::Incomplete(incomplete_mutation) => incomplete_mutation.remove(),
        }
    }
}

impl<'a> EffectiveMutationMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_remove_item_mutation(self.item_key, &mut reuse_eupdates)
            .unwrap();
    }
}

impl<'a> IncompleteMutationMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_remove_item_mutation(self.item_key, &mut reuse_eupdates)
            .unwrap();
    }
}
