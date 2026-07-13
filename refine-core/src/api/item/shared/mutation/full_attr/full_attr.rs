use crate::{ad::AAttrId, api::AttrId, sol::SolarSystem, ud::UItemId};

/// Full mutated attribute.
///
/// Attributes represented by this struct have their value impacted by mutation.
pub struct FullMAttr<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) attr_aid: AAttrId,
}
impl<'s> FullMAttr<'s> {
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

/// Full mutated attribute.
///
/// Attributes represented by this struct have their value impacted by mutation.
pub struct FullMAttrMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) item_uid: UItemId,
    pub(in crate::api) attr_aid: AAttrId,
}
impl<'s> FullMAttrMut<'s> {
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
