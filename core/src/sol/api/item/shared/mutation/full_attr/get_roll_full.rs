use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem, UnitInterval,
        api::{FullMAttr, FullMAttrMut},
        uad::item::normalize_a_attr_value,
    },
};

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
    let uad_item = sol.uad.items.get(item_key);
    if let Some(roll) = uad_item.get_mutation_data().unwrap().get_attr_rolls().get(a_attr_id) {
        return Some(*roll);
    }
    // If roll data was not available, calculate it using unmutated attribute value
    let mutation_range = uad_item
        .get_mutation_data()
        .unwrap()
        .get_cache()
        .unwrap()
        .get_a_mutator()
        .attr_mods
        .get(&a_attr_id)
        .unwrap();
    let value = uad_item.get_a_attr(&a_attr_id).unwrap();
    normalize_a_attr_value(value, value, mutation_range)
}
