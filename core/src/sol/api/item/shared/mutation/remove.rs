use crate::{
    def::ItemKey,
    sol::{
        SolarSystem,
        api::{EffectiveMutationMut, IncompleteMutationMut, MutationMut},
    },
    uad::{UadEffectUpdates, UadItem, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_item_mutation(
        &mut self,
        item_key: ItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        match self.uad.items.get(item_key) {
            UadItem::Drone(_) => self.internal_remove_drone_mutation(item_key, reuse_eupdates),
            UadItem::Module(_) => self.internal_remove_module_mutation(item_key, reuse_eupdates),
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
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol
            .internal_remove_item_mutation(self.item_key, &mut reuse_eupdates)
            .unwrap();
    }
}

impl<'a> IncompleteMutationMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol
            .internal_remove_item_mutation(self.item_key, &mut reuse_eupdates)
            .unwrap();
    }
}
