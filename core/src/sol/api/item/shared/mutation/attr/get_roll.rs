use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem, UnitInterval,
        api::{FullMAttr, FullMAttrMut, RawMAttr, RawMAttrMut},
    },
};

impl<'a> RawMAttr<'a> {
    pub fn get_roll(&self) -> UnitInterval {
        // Raw mutated attributes are not exposed for attributes without mutation data
        get_roll(self.sol, self.item_key, &self.a_attr_id).unwrap()
    }
}

impl<'a> RawMAttrMut<'a> {
    pub fn get_roll(&self) -> UnitInterval {
        // Raw mutated attributes are not exposed for attributes without mutation data
        get_roll(self.sol, self.item_key, &self.a_attr_id).unwrap()
    }
}

impl<'a> FullMAttr<'a> {
    pub fn get_roll(&self) -> Option<UnitInterval> {
        get_roll(self.sol, self.item_key, &self.a_attr_id)
    }
}

impl<'a> FullMAttrMut<'a> {
    pub fn get_roll(&self) -> Option<UnitInterval> {
        get_roll(self.sol, self.item_key, &self.a_attr_id)
    }
}

fn get_roll(sol: &SolarSystem, item_key: ItemKey, a_attr_id: &ad::AAttrId) -> Option<UnitInterval> {
    sol.uad
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(a_attr_id)
        .copied()
}
