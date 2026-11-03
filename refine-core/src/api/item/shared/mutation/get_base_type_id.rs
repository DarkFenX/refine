use crate::{EffectiveMutation, ItemTypeId, sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> EffectiveMutation<'s> {
    pub fn get_base_type_id(&self) -> ItemTypeId {
        self.sol.api_get_base_type_id(self.item_uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    fn api_get_base_type_id(&self, item_uid: UItemId) -> ItemTypeId {
        ItemTypeId::from_aid(
            self.u_data
                .items
                .get(item_uid)
                .get_mutation_data()
                .unwrap()
                .get_cache()
                .unwrap()
                .get_base_type_aid(),
        )
    }
}
