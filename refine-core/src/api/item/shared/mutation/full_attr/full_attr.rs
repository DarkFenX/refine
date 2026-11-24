use crate::{AttrId, ad::AAttrId, sol::SolCore, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Full mutated attribute.
///
/// Attributes represented by this struct have their value impacted by mutation.
pub struct FullMAttr<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) item_uid: UItemId,
    pub(super) attr_aid: AAttrId,
}
impl<'sol> FullMAttr<'sol> {
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        AttrId::from_aid(self.attr_aid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> FullMAttr<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, item_uid: UItemId, attr_aid: AAttrId) -> Self {
        Self {
            sol,
            item_uid,
            attr_aid,
        }
    }
}
