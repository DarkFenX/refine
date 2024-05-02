use crate::{
    ad::{AEffectAffecteeFilter, AOp},
    defs::EAttrId,
};

/// An adapted attribute modifier.
///
/// A modifier is an entity which specifies in detail which attributes on which items are going to
/// be affected, and how.
pub struct AEffectModifier {
    /// Refers an attribute on the modification source, which should be used as modification value.
    pub affector_attr_id: EAttrId,
    /// Operation to apply during the modification.
    pub op: AOp,
    /// Defines an affectee filter, that is a filter which defines which items will be affected by a
    /// modification.
    pub affectee_filter: AEffectAffecteeFilter,
    /// Refers an attribute which will be affected by modification.
    pub affectee_attr_id: EAttrId,
}
impl AEffectModifier {
    /// Make a new attribute modifier out of passed data.
    pub(crate) fn new(
        affector_attr_id: EAttrId,
        op: AOp,
        affectee_filter: AEffectAffecteeFilter,
        affectee_attr_id: EAttrId,
    ) -> Self {
        Self {
            affector_attr_id,
            op,
            affectee_filter,
            affectee_attr_id,
        }
    }
}
