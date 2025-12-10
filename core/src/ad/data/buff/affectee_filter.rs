use crate::ad::{AItemGrpId, AModifierSrq};

#[derive(Copy, Clone)]
pub enum ABuffAffecteeFilter {
    Direct,
    Loc,
    LocGrp(AItemGrpId),
    LocSrq(AModifierSrq),
}
