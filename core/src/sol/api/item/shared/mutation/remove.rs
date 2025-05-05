use crate::sol::{
    ItemKey, SolarSystem,
    api::{EffectiveMutationMut, IncompleteMutationMut, MutationMut},
    uad::item::UadItem,
};

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
        remove_mutation(self.sol, self.item_key)
    }
}

impl<'a> IncompleteMutationMut<'a> {
    pub fn remove(self) {
        remove_mutation(self.sol, self.item_key)
    }
}

fn remove_mutation(sol: &mut SolarSystem, item_key: ItemKey) {
    match sol.uad.items.get(item_key) {
        UadItem::Drone(_) => sol.internal_remove_drone_mutation(item_key).unwrap(),
        UadItem::Module(_) => sol.internal_remove_module_mutation(item_key).unwrap(),
        _ => panic!(),
    }
}
