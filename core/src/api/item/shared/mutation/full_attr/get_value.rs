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
        get_value(self.sol, self.item_key, &self.attr_aid)
    }
}

impl<'a> FullMAttrMut<'a> {
    /// Return mutated attribute value.
    pub fn get_value(&self) -> AttrVal {
        get_value(self.sol, self.item_key, &self.attr_aid)
    }
}

fn get_value(sol: &SolarSystem, item_key: UItemId, attr_aid: &AAttrId) -> AttrVal {
    let attr_key = sol.u_data.src.get_attr_rid_by_aid(attr_aid).unwrap();
    sol.u_data.items.get(item_key).get_attr(attr_key).unwrap()
}
