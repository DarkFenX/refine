use crate::{
    ad,
    sol::{
        AttrVal, ItemKey, SolarSystem,
        api::{FullMAttr, FullMAttrMut},
    },
};

impl<'a> FullMAttr<'a> {
    pub fn get_value(&self) -> AttrVal {
        get_value(self.sol, self.item_key, &self.a_attr_id)
    }
}

impl<'a> FullMAttrMut<'a> {
    pub fn get_value(&self) -> AttrVal {
        get_value(self.sol, self.item_key, &self.a_attr_id)
    }
}

fn get_value(sol: &SolarSystem, item_key: ItemKey, a_attr_id: &ad::AAttrId) -> AttrVal {
    sol.uad.items.get(item_key).get_a_attr(a_attr_id).unwrap()
}
