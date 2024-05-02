use crate::{
    ad::{AEffectDomain, AModifierSrq},
    defs::EItemGrpId,
};

/// Defines which items will be affected by a modifier.
pub enum AEffectAffecteeFilter {
    /// Single item modified, as specified by the domain.
    Direct(AEffectDomain),
    /// All items belonging to the domain are affected.
    Loc(AEffectDomain),
    /// All items located in the domain and belonging to the group are affected.
    LocGrp(AEffectDomain, EItemGrpId),
    /// All items located in the domain and having specified skill requirement are affected.
    LocSrq(AEffectDomain, AModifierSrq),
    /// All items belonging to the domain and having specified skill requirement are affected.
    OwnSrq(AModifierSrq),
}
