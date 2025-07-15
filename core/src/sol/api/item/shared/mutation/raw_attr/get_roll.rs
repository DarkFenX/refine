use crate::{
    ad,
    def::ItemKey,
    sol::{
        SolarSystem,
        api::{RawMAttr, RawMAttrMut},
    },
    util::UnitInterval,
};

impl<'a> RawMAttr<'a> {
    /// Return roll quality for the mutated attribute.
    pub fn get_roll(&self) -> UnitInterval {
        get_roll(self.sol, self.item_key, &self.a_attr_id)
    }
}

impl<'a> RawMAttrMut<'a> {
    /// Return roll quality for the mutated attribute.
    pub fn get_roll(&self) -> UnitInterval {
        get_roll(self.sol, self.item_key, &self.a_attr_id)
    }
}

fn get_roll(sol: &SolarSystem, item_key: ItemKey, a_attr_id: &ad::AAttrId) -> UnitInterval {
    sol.uad
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(a_attr_id)
        .copied()
        // Raw mutated attributes are not exposed for attributes without mutation data
        .unwrap()
}
