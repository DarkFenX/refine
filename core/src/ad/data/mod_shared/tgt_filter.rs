use crate::defs::EItemGrpId;

use super::{AModDomain, AModSrq};

/// Defines which items will be targeted for a modifier.
pub enum AModTgtFilter {
    /// Single item modified, as specified by the domain.
    Direct(AModDomain),
    /// All items belonging to the domain are affected.
    Loc(AModDomain),
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(AModDomain, EItemGrpId),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(AModDomain, AModSrq),
    /// All items belonging to the domain and having specified skill requirement are affected.
    OwnSrq(AModSrq),
}
