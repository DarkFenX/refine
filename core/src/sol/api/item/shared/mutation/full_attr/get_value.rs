use crate::{
    ad,
    def::AttrVal,
    sol::{
        SolarSystem,
        api::{FullMAttr, FullMAttrMut},
    },
    ud::UItemKey,
};

impl<'a> FullMAttr<'a> {
    /// Return mutated attribute value.
    pub fn get_value(&self) -> AttrVal {
        get_value(self.sol, self.item_key, &self.a_attr_id)
    }
}

impl<'a> FullMAttrMut<'a> {
    /// Return mutated attribute value.
    pub fn get_value(&self) -> AttrVal {
        get_value(self.sol, self.item_key, &self.a_attr_id)
    }
}

fn get_value(sol: &SolarSystem, item_key: UItemKey, a_attr_id: &ad::AAttrId) -> AttrVal {
    sol.u_data.items.get(item_key).get_a_attr(a_attr_id).unwrap()
}
