use crate::ad::{AAttrId, ABuffAffecteeFilter};

/// An attribute modifier.
///
/// Unlike effect modifiers, buff modifiers carry less data, since some of it resides on its parent
/// buff and some on the item applying the buff.
pub struct ABuffModifier {
    /// Defines an affectee filter, that is a filter which defines which items will be affected by
    /// modification.
    pub affectee_filter: ABuffAffecteeFilter,
    /// Refers an attribute which will be affected by modification.
    pub affectee_attr_id: AAttrId,
}
