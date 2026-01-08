use crate::{
    ad::AAttrId,
    api::{RawMAttr, RawMAttrMut},
    misc::UnitInterval,
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> RawMAttr<'a> {
    /// Return roll quality for the mutated attribute.
    pub fn get_roll(&self) -> UnitInterval {
        get_roll(self.sol, self.item_uid, &self.attr_aid)
    }
}

impl<'a> RawMAttrMut<'a> {
    /// Return roll quality for the mutated attribute.
    pub fn get_roll(&self) -> UnitInterval {
        get_roll(self.sol, self.item_uid, &self.attr_aid)
    }
}

fn get_roll(sol: &SolarSystem, item_uid: UItemId, attr_aid: &AAttrId) -> UnitInterval {
    sol.u_data
        .items
        .get(item_uid)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(attr_aid)
        .copied()
        // Raw mutated attributes are not exposed for attributes without mutation data
        .unwrap()
}
