use crate::ad::{AEffectLocation, AItemGrpId, AModifierSrq};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AEffectAffecteeFilter {
    Direct(AEffectLocation),
    Loc(AEffectLocation),
    LocGrp(AEffectLocation, AItemGrpId),
    LocSrq(AEffectLocation, AModifierSrq),
    OwnSrq(AModifierSrq),
}
