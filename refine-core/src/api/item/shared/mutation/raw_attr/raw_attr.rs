use crate::{AttrId, ad::AAttrId, sol::CoreInner, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Raw mutated attribute.
///
/// This attribute represents a user-defined mutation for the attribute. It means, when there is no
/// user-defined mutation for an attribute, you can't get corresponding raw mutated attribute.
///
/// Raw mutated attributes do not necessarily affect attributes of their parent item. However, they
/// are the only way to access item attribute mutations which are not available on current data
/// source.
pub struct RawMAttr<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) item_uid: UItemId,
    pub(super) attr_aid: AAttrId,
}
impl<'s> RawMAttr<'s> {
    /// Mutated attribute ID.
    pub fn get_attr_id(&self) -> AttrId {
        AttrId::from_aid(self.attr_aid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> RawMAttr<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, item_uid: UItemId, attr_aid: AAttrId) -> Self {
        Self {
            core,
            item_uid,
            attr_aid,
        }
    }
}
