use crate::{ad::AModSrq, defs::EItemGrpId, shr::ModDomain};

/// Defines which items will be targeted for a modifier.
pub enum AModTgtFilter {
    /// Single item modified, as specified by the domain.
    Direct(ModDomain),
    /// All items belonging to the domain are affected.
    Loc(ModDomain),
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(ModDomain, EItemGrpId),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(ModDomain, AModSrq),
    /// All items belonging to the domain and having specified skill requirement are affected.
    OwnSrq(ModDomain, AModSrq),
}
