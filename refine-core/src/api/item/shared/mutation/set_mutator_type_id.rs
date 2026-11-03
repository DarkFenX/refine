use crate::{
    DormantMutation, EffectiveMutation, ItemTypeId, Mutation, SolarSystem,
    ad::AItemId,
    ud::{UItem, UItemId, err::ItemMutatedError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Mutation<'s> {
    pub fn set_mutator_type_id(self, mutator_type_id: ItemTypeId) -> Mutation<'s> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.set_mutator_type_id(mutator_type_id),
            Self::Dormant(dormant_mutation) => dormant_mutation.set_mutator_type_id(mutator_type_id),
        }
    }
}

impl<'s> EffectiveMutation<'s> {
    pub fn set_mutator_type_id(self, mutator_type_id: ItemTypeId) -> Mutation<'s> {
        self.sol
            .internal_set_mutator_aid(self.item_uid, mutator_type_id.into_aid())
            .unwrap();
        self.sol.api_get_item_mutation(self.item_uid).unwrap()
    }
}

impl<'s> DormantMutation<'s> {
    pub fn set_mutator_type_id(self, mutator_type_id: ItemTypeId) -> Mutation<'s> {
        self.sol
            .internal_set_mutator_aid(self.item_uid, mutator_type_id.into_aid())
            .unwrap();
        self.sol.api_get_item_mutation(self.item_uid).unwrap()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_set_mutator_aid(
        &mut self,
        item_uid: UItemId,
        mutator_type_aid: AItemId,
    ) -> Result<(), ItemMutatedError> {
        let u_item = self.u_data.items.get_mut(item_uid);
        match u_item {
            UItem::Drone(..) => self.internal_set_drone_mutator_type_aid(item_uid, mutator_type_aid),
            UItem::Module(..) => self.internal_set_module_mutator_aid(item_uid, mutator_type_aid),
            _ => unreachable!("unmutable item kind is used to change mutator ID"),
        }
    }
}
