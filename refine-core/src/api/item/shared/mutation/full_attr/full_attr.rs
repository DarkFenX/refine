use crate::{AttrId, SolarSystem, ad::AAttrId, ud::UItemId};

/// Full mutated attribute.
///
/// Attributes represented by this struct have their value impacted by mutation.
pub struct FullMAttr<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) item_uid: UItemId,
    pub(super) attr_aid: AAttrId,
}
impl<'s> FullMAttr<'s> {
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
