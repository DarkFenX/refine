use crate::handler_json::data::{CEffectDomain, CModSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CEffectAffecteeFilter {
    Direct(CEffectDomain),
    Loc(CEffectDomain),
    LocGrp(CEffectDomain, rc::EItemGrpId),
    LocSrq(CEffectDomain, CModSrq),
    OwnSrq(CModSrq),
}
impl From<&rc::ad::AEffectAffecteeFilter> for CEffectAffecteeFilter {
    fn from(effect_affectee_filter: &rc::ad::AEffectAffecteeFilter) -> Self {
        match effect_affectee_filter {
            rc::ad::AEffectAffecteeFilter::Direct(dom) => Self::Direct(dom.into()),
            rc::ad::AEffectAffecteeFilter::Loc(dom) => Self::Loc(dom.into()),
            rc::ad::AEffectAffecteeFilter::LocGrp(dom, grp) => Self::LocGrp(dom.into(), *grp),
            rc::ad::AEffectAffecteeFilter::LocSrq(dom, srq) => Self::LocSrq(dom.into(), srq.into()),
            rc::ad::AEffectAffecteeFilter::OwnSrq(srq) => Self::OwnSrq(srq.into()),
        }
    }
}
impl Into<rc::ad::AEffectAffecteeFilter> for &CEffectAffecteeFilter {
    fn into(self) -> rc::ad::AEffectAffecteeFilter {
        match self {
            CEffectAffecteeFilter::Direct(dom) => rc::ad::AEffectAffecteeFilter::Direct(dom.into()),
            CEffectAffecteeFilter::Loc(dom) => rc::ad::AEffectAffecteeFilter::Loc(dom.into()),
            CEffectAffecteeFilter::LocGrp(dom, grp) => rc::ad::AEffectAffecteeFilter::LocGrp(dom.into(), *grp),
            CEffectAffecteeFilter::LocSrq(dom, srq) => rc::ad::AEffectAffecteeFilter::LocSrq(dom.into(), srq.into()),
            CEffectAffecteeFilter::OwnSrq(srq) => rc::ad::AEffectAffecteeFilter::OwnSrq(srq.into()),
        }
    }
}
