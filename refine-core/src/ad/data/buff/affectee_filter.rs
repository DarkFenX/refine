use crate::ad::{AItemGrpId, AModifierSrq};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone)]
pub enum ABuffAffecteeFilter {
    Direct,
    Loc,
    LocGrp(AItemGrpId),
    LocSrq(AModifierSrq),
}
