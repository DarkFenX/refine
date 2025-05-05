use crate::{
    ad,
    sol::{AttrId, ItemKey, SolarSystem},
};

/// Full mutated attribute.
///
/// Full mutated attributes are exposed by an effective item mutation, and are limited to those
/// defined by mutator on current data source. They do not necessarily contain user-defined mutation
/// data, but they provide full set of functionality to mutate an attribute and get its value.
pub struct FullMAttr<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) item_key: ItemKey,
    pub(in crate::sol::api) a_attr_id: ad::AAttrId,
}
impl<'a> FullMAttr<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, item_key: ItemKey, a_attr_id: ad::AAttrId) -> Self {
        Self {
            sol,
            item_key,
            a_attr_id,
        }
    }
    pub fn get_attr_id(&self) -> AttrId {
        self.a_attr_id
    }
}

/// Full mutated attribute.
///
/// Full mutated attributes are exposed by an effective item mutation, and are limited to those
/// defined by mutator on current data source. They do not necessarily contain user-defined mutation
/// data, but they provide full set of functionality to mutate an attribute and get its value.
pub struct FullMAttrMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) item_key: ItemKey,
    pub(in crate::sol::api) a_attr_id: ad::AAttrId,
}
impl<'a> FullMAttrMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, item_key: ItemKey, a_attr_id: ad::AAttrId) -> Self {
        Self {
            sol,
            item_key,
            a_attr_id,
        }
    }
    pub fn get_attr_id(&self) -> AttrId {
        self.a_attr_id
    }
}
