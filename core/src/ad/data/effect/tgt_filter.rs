use crate::{
    ad::{AEffectDomain, AModSrq},
    defs::EItemGrpId,
};

/// Defines which items will be targeted for a modifier.
pub enum AEffectTgtFilter {
    /// Single item modified, as specified by the domain.
    Direct(AEffectDomain),
    /// All items belonging to the domain are affected.
    Loc(AEffectDomain),
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(AEffectDomain, EItemGrpId),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(AEffectDomain, AModSrq),
    /// All items belonging to the domain and having specified skill requirement are affected.
    OwnSrq(AModSrq),
}
