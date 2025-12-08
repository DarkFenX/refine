use crate::ad::{AItemGrpId, AModifierSrq};

/// Defines which items will be affected by a modifier.
#[derive(Copy, Clone)]
pub enum ABuffAffecteeFilter {
    /// Single item modified, as specified by the location.
    Direct,
    /// All items belonging to the location are affected.
    Loc,
    /// All items located in the location and belonging to the group are affected.
    LocGrp(AItemGrpId),
    /// All items located in the location and having specified skill requirement are affected.
    LocSrq(AModifierSrq),
}
