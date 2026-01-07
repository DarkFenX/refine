use crate::{
    ad::AAttrId,
    api::{RawMAttr, RawMAttrMut},
    sol::SolarSystem,
    ud::UItemId,
    util::UnitInterval,
};

impl<'a> RawMAttr<'a> {
    /// Return roll quality for the mutated attribute.
    pub fn get_roll(&self) -> UnitInterval {
        get_roll(self.sol, self.item_key, &self.attr_aid)
    }
}

impl<'a> RawMAttrMut<'a> {
    /// Return roll quality for the mutated attribute.
    pub fn get_roll(&self) -> UnitInterval {
        get_roll(self.sol, self.item_key, &self.attr_aid)
    }
}

fn get_roll(sol: &SolarSystem, item_key: UItemId, attr_aid: &AAttrId) -> UnitInterval {
    sol.u_data
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(attr_aid)
        .copied()
        // Raw mutated attributes are not exposed for attributes without mutation data
        .unwrap()
}
