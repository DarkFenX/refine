use crate::{ad::AModTgtFilter, defs::EAttrId, shr::ModOp, util::Named};

/// An adapted attribute modifier.
///
/// A modifier is an entity which specifies in detail which attributes on which items are going to
/// be affected, and how.
pub struct AEffectAttrMod {
    /// Refers an attribute on the modification source, which should be used as modification value.
    pub src_attr_id: EAttrId,
    /// Operation to apply during the modification.
    pub op: ModOp,
    /// Defines a target filter, that is a filter which defines which items will be targeted for
    /// modification.
    pub tgt_filter: AModTgtFilter,
    /// Refers an attribute, whose value will be target for modification.
    pub tgt_attr_id: EAttrId,
}
impl AEffectAttrMod {
    /// Make a new attribute modifier out of passed data.
    pub(crate) fn new(src_attr_id: EAttrId, op: ModOp, tgt_filter: AModTgtFilter, tgt_attr_id: EAttrId) -> Self {
        Self {
            src_attr_id,
            op,
            tgt_filter,
            tgt_attr_id,
        }
    }
}
