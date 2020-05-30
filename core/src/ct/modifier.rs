use crate::{
    consts::{ModAfeeFilter, ModAggrMode, ModOp},
    defines::ReeInt,
};

/// A standard attribute modifier.
///
/// A modifier is an entity which specifies in detail which attributes on which items are going to
/// be affected, and how. A standard modifier always uses an attribute value stored on the affector
/// (the item which carries the effect with the modifier) as modification value.
#[derive(Debug)]
pub struct AttrMod {
    /// Refers an attribute on the affector, which should be used as modification value.
    pub afor_attr_id: ReeInt,
    /// Defines how multiple modifications applied to the same attribute value are aggregated.
    pub aggr_mode: ModAggrMode,
    /// Operation to apply during the modification.
    pub op: ModOp,
    /// Defines an affectee filter, that is a filter which defines which items will be affected.
    pub afee_filter: ModAfeeFilter,
    /// Refers an attribute, whose value will be affected on the affectee.
    pub afee_attr_id: ReeInt,
}
impl AttrMod {
    /// Make a new standard attribute modifier out of passed data.
    pub fn new(
        afor_attr_id: ReeInt,
        aggr_mode: ModAggrMode,
        op: ModOp,
        afee_filter: ModAfeeFilter,
        afee_attr_id: ReeInt,
    ) -> AttrMod {
        AttrMod {
            afor_attr_id,
            aggr_mode,
            op,
            afee_filter,
            afee_attr_id,
        }
    }
}
