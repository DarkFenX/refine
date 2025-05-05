use crate::sol::{
    ItemKey, ItemTypeId, SolarSystem,
    api::{EffectiveMutation, EffectiveMutationMut},
};

impl SolarSystem {
    fn api_get_base_type_id(&self, item_key: ItemKey) -> ItemTypeId {
        self.uad
            .items
            .get(item_key)
            .get_mutation_data()
            .unwrap()
            .get_cache()
            .unwrap()
            .get_base_a_item_id()
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
