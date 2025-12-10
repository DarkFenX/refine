use crate::ad::{AEffectLocation, AItemGrpId, AModifierSrq};

#[derive(Copy, Clone)]
pub enum AEffectAffecteeFilter {
    Direct(AEffectLocation),
    Loc(AEffectLocation),
    LocGrp(AEffectLocation, AItemGrpId),
    LocSrq(AEffectLocation, AModifierSrq),
    OwnSrq(AModifierSrq),
}
