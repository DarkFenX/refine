use crate::{ad::ModAfeeFilter, defs::EAttrId, shr::ModOp, util::Named};

/// An adapted attribute modifier.
///
/// A modifier is an entity which specifies in detail which attributes on which items are going to
/// be affected, and how.
pub struct AEffectAttrMod {
    /// Refers an attribute on the affector, which should be used as modification value.
    pub afor_attr_id: EAttrId,
    /// Operation to apply during the modification.
    pub op: ModOp,
    /// Defines an affectee filter, that is a filter which defines which items will be affected.
    pub afee_filter: ModAfeeFilter,
    /// Refers an attribute, whose value will be affected on the affectee.
    pub afee_attr_id: EAttrId,
}
impl AEffectAttrMod {
    /// Make a new attribute modifier out of passed data.
    pub(crate) fn new(afor_attr_id: EAttrId, op: ModOp, afee_filter: ModAfeeFilter, afee_attr_id: EAttrId) -> Self {
        Self {
            afor_attr_id,
            op,
            afee_filter,
            afee_attr_id,
        }
    }
}
impl Named for AEffectAttrMod {
    fn get_name() -> &'static str {
        "AAttrMod"
    }
}
