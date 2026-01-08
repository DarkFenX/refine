use crate::{
    ad::AAttrId,
    api::{FullMAttr, FullMAttrMut},
    misc::Value,
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> FullMAttr<'a> {
    /// Return mutated attribute value.
    pub fn get_value(&self) -> Value {
        get_value(self.sol, self.item_uid, &self.attr_aid)
    }
}

impl<'a> FullMAttrMut<'a> {
    /// Return mutated attribute value.
    pub fn get_value(&self) -> Value {
        get_value(self.sol, self.item_uid, &self.attr_aid)
    }
}

fn get_value(sol: &SolarSystem, item_uid: UItemId, attr_aid: &AAttrId) -> Value {
    let attr_rid = sol.u_data.src.get_attr_rid_by_aid(attr_aid).unwrap();
    sol.u_data.items.get(item_uid).get_attr(attr_rid).unwrap()
}
