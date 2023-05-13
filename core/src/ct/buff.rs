use crate::{
    consts::{ModAfeeFilter, ModAggrMode, ModOp},
    defs::ReeInt,
    util::Named,
};

/// Represents a dogma buff.
///
/// A dogma buff applies modifications to multiple ships, and the modifications stick for some time.
/// For instance, fleet effects are implemented as dogma buffs.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Buff {
    /// Buff ID.
    pub id: ReeInt,
    /// Defines how multiple modifications of the same attribute value are aggregated.
    pub aggr_mode: ModAggrMode,
    /// Operation to use when applying the buff's modifiers.
    pub op: ModOp,
    /// Attribute modifiers carried by the buff
    pub mods: Vec<BuffAttrMod>,
}
impl Buff {
    /// Make a new dogma buff out of passed data.
    pub(crate) fn new(id: ReeInt, aggr_mode: ModAggrMode, op: ModOp, mods: Vec<BuffAttrMod>) -> Self {
        Self {
            id,
            aggr_mode,
            op,
            mods,
        }
    }
}
impl Named for Buff {
    fn get_name() -> &'static str {
        "ct::Buff"
    }
}

/// A buff-specific attribute modifier.
///
/// Unlike the effect modifier, the buff modifier carries less data, since some of it resides on its
/// parent buff and some on the entity applying the buff.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BuffAttrMod {
    /// Defines an affectee filter, that is a filter which defines which items will be affected.
    pub afee_filter: ModAfeeFilter,
    /// Refers an attribute, whose value will be affected on the affectee.
    pub afee_attr_id: ReeInt,
}
impl BuffAttrMod {
    /// Make a new buff-specific attribute modifier out of passed data.
    pub(crate) fn new(afee_filter: ModAfeeFilter, afee_attr_id: ReeInt) -> Self {
        Self {
            afee_filter,
            afee_attr_id,
        }
    }
}
impl Named for BuffAttrMod {
    fn get_name() -> &'static str {
        "ct::BuffAttrMod"
    }
}
