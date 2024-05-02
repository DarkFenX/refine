use crate::{ad::AModifierSrq, defs::EItemGrpId};

/// Defines which items will be affected by a modifier.
pub enum ABuffAffecteeFilter {
    /// Single item modified, as specified by the domain.
    Direct,
    /// All items belonging to the domain are affected.
    Loc,
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(EItemGrpId),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(AModifierSrq),
}
