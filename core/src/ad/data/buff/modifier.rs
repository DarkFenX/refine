use crate::{ad::ABuffAffecteeFilter, defs::EAttrId};

/// An attribute modifier.
///
/// Unlike effect modifiers, buff modifiers carry less data, since some of it resides on its parent
/// buff and some on the item applying the buff.
pub struct ABuffModifier {
    /// Defines an affectee filter, that is a filter which defines which items will be affected by
    /// modification.
    pub affectee_filter: ABuffAffecteeFilter,
    /// Refers an attribute which will be affected by modification.
    pub affectee_attr_id: EAttrId,
}
impl ABuffModifier {
    /// Make a new buff-specific attribute modifier out of passed data.
    pub(crate) fn new(affectee_filter: ABuffAffecteeFilter, affectee_attr_id: EAttrId) -> Self {
        Self {
            affectee_filter,
            affectee_attr_id,
        }
    }
}
