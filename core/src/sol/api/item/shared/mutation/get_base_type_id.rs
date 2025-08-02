use crate::{
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{EffectiveMutation, EffectiveMutationMut},
    },
    ud::UItemKey,
};

impl SolarSystem {
    fn api_get_base_type_id(&self, item_key: UItemKey) -> ItemTypeId {
        self.u_data
            .items
            .get(item_key)
            .get_mutation_data()
            .unwrap()
            .get_cache()
            .unwrap()
            .get_base_type_id()
    }
}

impl<'a> EffectiveMutation<'a> {
    pub fn get_base_type_id(&self) -> ItemTypeId {
        self.sol.api_get_base_type_id(self.item_key)
    }
}
impl<'a> EffectiveMutationMut<'a> {
    pub fn get_base_type_id(&self) -> ItemTypeId {
        self.sol.api_get_base_type_id(self.item_key)
    }
}
