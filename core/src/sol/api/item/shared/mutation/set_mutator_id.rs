use crate::sol::{
    ItemKey, ItemTypeId, SolarSystem,
    api::{EffectiveMutationMut, IncompleteMutationMut, MutationMut},
    err::ItemMutatedError,
    uad::item::UadItem,
};

impl SolarSystem {
    pub(in crate::sol) fn internal_set_mutator_id(
        &mut self,
        item_key: ItemKey,
        mutator_id: ItemTypeId,
    ) -> Result<(), ItemMutatedError> {
        let uad_item = self.uad.items.get_mut(item_key);
        match uad_item {
            UadItem::Drone(_) => self.internal_set_drone_a_mutator_id(item_key, mutator_id),
            UadItem::Module(_) => self.internal_set_module_a_mutator_id(item_key, mutator_id),
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
        self.sol.internal_set_mutator_id(self.item_key, mutator_id).unwrap();
        self.sol.api_get_item_mutation_mut(self.item_key).unwrap()
    }
}

impl<'a> IncompleteMutationMut<'a> {
    pub fn set_mutator_id(self, mutator_id: ItemTypeId) -> MutationMut<'a> {
        self.sol.internal_set_mutator_id(self.item_key, mutator_id).unwrap();
        self.sol.api_get_item_mutation_mut(self.item_key).unwrap()
    }
}
