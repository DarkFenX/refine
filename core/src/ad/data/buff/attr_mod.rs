use crate::{ad::AModTgtFilter, defs::EAttrId};

/// An attribute modifier.
///
/// Unlike effect modifiers, buff modifiers carry less data, since some of it resides on its parent
/// buff and some on the item applying the buff.
pub struct ABuffAttrMod {
    /// Defines a target filter, that is a filter which defines which items will be targeted for
    /// modification.
    pub tgt_filter: AModTgtFilter,
    /// Refers an attribute, whose value will be target for modification.
    pub tgt_attr_id: EAttrId,
}
impl ABuffAttrMod {
    /// Make a new buff-specific attribute modifier out of passed data.
    pub(crate) fn new(tgt_filter: AModTgtFilter, tgt_attr_id: EAttrId) -> Self {
        Self {
            tgt_filter,
            tgt_attr_id,
        }
    }
}
