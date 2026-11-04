use crate::{AttrId, ad::AAttrId, sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Full mutated attribute.
///
/// Attributes represented by this struct have their value impacted by mutation.
pub struct FullMAttr<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) item_uid: UItemId,
    pub(super) attr_aid: AAttrId,
}
impl<'s> FullMAttr<'s> {
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        AttrId::from_aid(self.attr_aid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FullMAttr<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, item_uid: UItemId, attr_aid: AAttrId) -> Self {
        Self {
            core,
            item_uid,
            attr_aid,
        }
    }
}
