use crate::ad::{AEffectLocation, AItemGrpId, AModifierSrq};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AEffectAffecteeFilter {
    Direct(AEffectLocation),
    Loc(AEffectLocation),
    LocGrp(AEffectLocation, AItemGrpId),
    LocSrq(AEffectLocation, AModifierSrq),
    OwnSrq(AModifierSrq),
}
