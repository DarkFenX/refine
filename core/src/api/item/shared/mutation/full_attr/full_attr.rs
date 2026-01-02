use crate::{ad, api::AttrId, sol::SolarSystem, ud::UItemId};

/// Full mutated attribute.
///
/// Attributes represented by this struct have their value impacted by mutation.
pub struct FullMAttr<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) item_key: UItemId,
    pub(in crate::api) a_attr_id: ad::AAttrId,
}
impl<'a> FullMAttr<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, item_key: UItemId, a_attr_id: ad::AAttrId) -> Self {
        Self {
            sol,
            item_key,
            a_attr_id,
        }
    }
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        self.a_attr_id.into()
    }
}

/// Full mutated attribute.
///
/// Attributes represented by this struct have their value impacted by mutation.
pub struct FullMAttrMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) item_key: UItemId,
    pub(in crate::api) a_attr_id: ad::AAttrId,
}
impl<'a> FullMAttrMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, item_key: UItemId, a_attr_id: ad::AAttrId) -> Self {
        Self {
            sol,
            item_key,
            a_attr_id,
        }
    }
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        self.a_attr_id.into()
    }
}
