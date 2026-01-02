use crate::{
    ad::AAttrId,
    api::{FullMAttr, FullMAttrMut},
    def::AttrVal,
    sol::SolarSystem,
    ud::UItemId,
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

fn get_value(sol: &SolarSystem, item_key: UItemId, a_attr_id: &AAttrId) -> AttrVal {
    let attr_key = sol.u_data.src.get_attr_key_by_id(a_attr_id).unwrap();
    sol.u_data.items.get(item_key).get_attr(attr_key).unwrap()
}
