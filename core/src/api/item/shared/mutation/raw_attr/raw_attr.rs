use crate::{ad::AAttrId, misc::AttrId, sol::SolarSystem, ud::UItemKey};

/// Raw mutated attribute.
///
/// This attribute represents a user-defined mutation for the attribute. It means, when there is no
/// user-defined mutation for an attribute, you can't get corresponding raw mutated attribute.
///
/// Raw mutated attributes do not necessarily affect attributes of their parent item. However, they
/// are the only way to access item attribute mutations which are not available on current data
/// source.
pub struct RawMAttr<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) item_key: UItemKey,
    pub(in crate::api) a_attr_id: AAttrId,
}
impl<'a> RawMAttr<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, item_key: UItemKey, a_attr_id: AAttrId) -> Self {
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

/// Raw mutated attribute.
///
/// This attribute represents a user-defined mutation for the attribute. It means, when there is no
/// user-defined mutation for an attribute, you can't get corresponding raw mutated attribute.
///
/// Raw mutated attributes do not necessarily affect attributes of their parent item. However, they
/// are the only way to access item attribute mutations which are not available on current data
/// source.
pub struct RawMAttrMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) item_key: UItemKey,
    pub(in crate::api) a_attr_id: AAttrId,
}
impl<'a> RawMAttrMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, item_key: UItemKey, a_attr_id: AAttrId) -> Self {
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
