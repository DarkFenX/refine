use crate::{
    ad::{AEffectLocation, AModifierSrq},
    defs::EItemGrpId,
};

/// Defines which items will be affected by a modifier.
pub enum AEffectAffecteeFilter {
    /// Single item modified, as specified by the location.
    Direct(AEffectLocation),
    /// All items belonging to the location are affected.
    Loc(AEffectLocation),
    /// All items located in the location and belonging to the group are affected.
    LocGrp(AEffectLocation, EItemGrpId),
    /// All items located in the location and having specified skill requirement are affected.
    LocSrq(AEffectLocation, AModifierSrq),
    /// All items belonging to the location and having specified skill requirement are affected.
    OwnSrq(AModifierSrq),
}
