use super::{super::shared::CModifierSrq, location::CEffectLocation};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum CEffectAffecteeFilter {
    Direct(CEffectLocation),
    Loc(CEffectLocation),
    LocGrp(CEffectLocation, i32),
    LocSrq(CEffectLocation, CModifierSrq),
    OwnSrq(CModifierSrq),
}
impl CEffectAffecteeFilter {
    pub(super) fn from_adapted(a_effect_affectee_filter: &rc::ad::AEffectAffecteeFilter) -> Self {
        match a_effect_affectee_filter {
            rc::ad::AEffectAffecteeFilter::Direct(loc) => Self::Direct(CEffectLocation::from_adapted(loc)),
            rc::ad::AEffectAffecteeFilter::Loc(loc) => Self::Loc(CEffectLocation::from_adapted(loc)),
            rc::ad::AEffectAffecteeFilter::LocGrp(loc, grp) => {
                Self::LocGrp(CEffectLocation::from_adapted(loc), grp.into_i32())
            }
            rc::ad::AEffectAffecteeFilter::LocSrq(loc, srq) => {
                Self::LocSrq(CEffectLocation::from_adapted(loc), CModifierSrq::from_adapted(srq))
            }
            rc::ad::AEffectAffecteeFilter::OwnSrq(srq) => Self::OwnSrq(CModifierSrq::from_adapted(srq)),
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AEffectAffecteeFilter {
        match self {
            Self::Direct(loc) => rc::ad::AEffectAffecteeFilter::Direct(loc.into_adapted()),
            Self::Loc(loc) => rc::ad::AEffectAffecteeFilter::Loc(loc.into_adapted()),
            Self::LocGrp(loc, grp) => {
                rc::ad::AEffectAffecteeFilter::LocGrp(loc.into_adapted(), rc::ad::AItemGrpId::from_i32(grp))
            }
            Self::LocSrq(loc, srq) => rc::ad::AEffectAffecteeFilter::LocSrq(loc.into_adapted(), srq.into_adapted()),
            Self::OwnSrq(srq) => rc::ad::AEffectAffecteeFilter::OwnSrq(srq.into_adapted()),
        }
    }
}
