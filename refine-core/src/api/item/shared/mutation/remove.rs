use crate::{
    DormantMutation, EffectiveMutation, Mutation,
    sol::CoreInner,
    ud::{UItem, UItemId, err::ItemMutatedError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Mutation<'s> {
    pub fn remove(self) {
        match self {
            Mutation::Effective(effective_mutation) => effective_mutation.remove(),
            Mutation::Dormant(dormant_mutation) => dormant_mutation.remove(),
        }
    }
}

impl<'s> EffectiveMutation<'s> {
    pub fn remove(self) {
        self.core.internal_remove_item_mutation(self.item_uid).unwrap();
    }
}

impl<'s> DormantMutation<'s> {
    pub fn remove(self) {
        self.core.internal_remove_item_mutation(self.item_uid).unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_remove_item_mutation(&mut self, item_uid: UItemId) -> Result<(), ItemMutatedError> {
        match self.u_data.items.get(item_uid) {
            UItem::Drone(..) => self.internal_remove_drone_mutation(item_uid),
            UItem::Module(..) => self.internal_remove_module_mutation(item_uid),
            _ => unreachable!("unmutable item kind is used in mutation"),
        }
    }
}
