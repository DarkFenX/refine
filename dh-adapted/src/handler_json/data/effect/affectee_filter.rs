use crate::handler_json::data::{CEffectLocation, CModSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CEffectAffecteeFilter {
    Direct(CEffectLocation),
    Loc(CEffectLocation),
    LocGrp(CEffectLocation, rc::EItemGrpId),
    LocSrq(CEffectLocation, CModSrq),
    OwnSrq(CModSrq),
}
impl From<&rc::ad::AEffectAffecteeFilter> for CEffectAffecteeFilter {
    fn from(effect_affectee_filter: &rc::ad::AEffectAffecteeFilter) -> Self {
        match effect_affectee_filter {
            rc::ad::AEffectAffecteeFilter::Direct(loc) => Self::Direct(loc.into()),
            rc::ad::AEffectAffecteeFilter::Loc(loc) => Self::Loc(loc.into()),
            rc::ad::AEffectAffecteeFilter::LocGrp(loc, grp) => Self::LocGrp(loc.into(), *grp),
            rc::ad::AEffectAffecteeFilter::LocSrq(loc, srq) => Self::LocSrq(loc.into(), srq.into()),
            rc::ad::AEffectAffecteeFilter::OwnSrq(srq) => Self::OwnSrq(srq.into()),
        }
    }
}
impl Into<rc::ad::AEffectAffecteeFilter> for &CEffectAffecteeFilter {
    fn into(self) -> rc::ad::AEffectAffecteeFilter {
        match self {
            CEffectAffecteeFilter::Direct(loc) => rc::ad::AEffectAffecteeFilter::Direct(loc.into()),
            CEffectAffecteeFilter::Loc(loc) => rc::ad::AEffectAffecteeFilter::Loc(loc.into()),
            CEffectAffecteeFilter::LocGrp(loc, grp) => rc::ad::AEffectAffecteeFilter::LocGrp(loc.into(), *grp),
            CEffectAffecteeFilter::LocSrq(loc, srq) => rc::ad::AEffectAffecteeFilter::LocSrq(loc.into(), srq.into()),
            CEffectAffecteeFilter::OwnSrq(srq) => rc::ad::AEffectAffecteeFilter::OwnSrq(srq.into()),
        }
    }
}
