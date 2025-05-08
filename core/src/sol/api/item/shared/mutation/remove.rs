use crate::sol::{
    ItemKey, SolarSystem,
    api::{EffectiveMutationMut, IncompleteMutationMut, MutationMut},
    err::ItemMutatedError,
    uad::item::UadItem,
};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_item_mutation(&mut self, item_key: ItemKey) -> Result<(), ItemMutatedError> {
        match self.uad.items.get(item_key) {
            UadItem::Drone(_) => self.internal_remove_drone_mutation(item_key),
            UadItem::Module(_) => self.internal_remove_module_mutation(item_key),
            _ => unreachable!(),
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
        self.sol.internal_remove_item_mutation(self.item_key).unwrap();
    }
}

impl<'a> IncompleteMutationMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_item_mutation(self.item_key).unwrap();
    }
}
