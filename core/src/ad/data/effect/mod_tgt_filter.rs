use crate::{ad::ModSrq, defs::EItemGrpId, shr::ModDomain};

/// Defines which items will be affected by a modifier.
#[derive(Debug)]
pub enum ModAfeeFilter {
    /// Single item modified, as specified by the domain.
    Direct(ModDomain),
    /// All items belonging to the domain are affected.
    Loc(ModDomain),
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(ModDomain, EItemGrpId),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(ModDomain, ModSrq),
    /// All items belonging to the domain and having specified skill requirement are affected.
    OwnSrq(ModDomain, ModSrq),
}
