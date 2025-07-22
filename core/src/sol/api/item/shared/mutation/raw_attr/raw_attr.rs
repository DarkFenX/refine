use crate::{ad, def::AttrId, sol::SolarSystem, uad::UadItemKey};

/// Raw mutated attribute.
///
/// This attribute represents a user-defined mutation for the attribute. It means, when there is no
/// user-defined mutation for an attribute, you can't get corresponding raw mutated attribute.
///
/// Raw mutated attributes do not necessarily affect attributes of their parent item. However, they
/// are the only way to access item attribute mutations which are not available on current data
/// source.
pub struct RawMAttr<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) item_key: UadItemKey,
    pub(in crate::sol::api) a_attr_id: ad::AAttrId,
}
impl<'a> RawMAttr<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, item_key: UadItemKey, a_attr_id: ad::AAttrId) -> Self {
        Self {
            sol,
            item_key,
            a_attr_id,
        }
    }
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        self.a_attr_id
    }
}

/// Raw mutated attribute.
///
/// This attribute represents a user-defined mutation for the attribute. It means, when there is no
/// user-defined mutation for an attribute, you can't get corresponding raw mutated attribute.
///
/// Raw mutated attributes do not necessarily affect attributes of their parent item. However, they
/// are the only way to access item attribute mutations which are not available on current data
/// source.
pub struct RawMAttrMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) item_key: UadItemKey,
    pub(in crate::sol::api) a_attr_id: ad::AAttrId,
}
impl<'a> RawMAttrMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, item_key: UadItemKey, a_attr_id: ad::AAttrId) -> Self {
        Self {
            sol,
            item_key,
            a_attr_id,
        }
    }
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        self.a_attr_id
    }
}
