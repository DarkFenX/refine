use crate::{
    api::{EffectiveMutation, EffectiveMutationMut, ItemTypeId},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
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

impl<'a> EffectiveMutation<'a> {
    pub fn get_base_type_id(&self) -> ItemTypeId {
        self.sol.api_get_base_type_id(self.item_uid)
    }
}
impl<'a> EffectiveMutationMut<'a> {
    pub fn get_base_type_id(&self) -> ItemTypeId {
        self.sol.api_get_base_type_id(self.item_uid)
    }
}
