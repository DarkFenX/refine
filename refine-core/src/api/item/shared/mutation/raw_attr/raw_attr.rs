use crate::{ad::AAttrId, api::AttrId, sol::SolarSystem, ud::UItemId};

/// Raw mutated attribute.
///
/// This attribute represents a user-defined mutation for the attribute. It means, when there is no
/// user-defined mutation for an attribute, you can't get corresponding raw mutated attribute.
///
/// Raw mutated attributes do not necessarily affect attributes of their parent item. However, they
/// are the only way to access item attribute mutations which are not available on current data
/// source.
pub struct RawMAttr<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) attr_aid: AAttrId,
}
impl<'s> RawMAttr<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, item_uid: UItemId, attr_aid: AAttrId) -> Self {
        Self {
            sol,
            item_uid,
            attr_aid,
        }
    }
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        AttrId::from_aid(self.attr_aid)
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
pub struct RawMAttrMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) attr_aid: AAttrId,
}
impl<'s> RawMAttrMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, item_uid: UItemId, attr_aid: AAttrId) -> Self {
        Self {
            sol,
            item_uid,
            attr_aid,
        }
    }
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        AttrId::from_aid(self.attr_aid)
    }
}
