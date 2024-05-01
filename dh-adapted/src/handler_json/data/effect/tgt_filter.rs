use crate::handler_json::data::{CEffectDomain, CModSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CEffectTgtFilter {
    Direct(CEffectDomain),
    Loc(CEffectDomain),
    LocGrp(CEffectDomain, rc::EItemGrpId),
    LocSrq(CEffectDomain, CModSrq),
    OwnSrq(CModSrq),
}
impl From<&rc::ad::AEffectAffecteeFilter> for CEffectTgtFilter {
    fn from(effect_tgt_filter: &rc::ad::AEffectAffecteeFilter) -> Self {
        match effect_tgt_filter {
            rc::ad::AEffectAffecteeFilter::Direct(dom) => Self::Direct(dom.into()),
            rc::ad::AEffectAffecteeFilter::Loc(dom) => Self::Loc(dom.into()),
            rc::ad::AEffectAffecteeFilter::LocGrp(dom, grp) => Self::LocGrp(dom.into(), *grp),
            rc::ad::AEffectAffecteeFilter::LocSrq(dom, srq) => Self::LocSrq(dom.into(), srq.into()),
            rc::ad::AEffectAffecteeFilter::OwnSrq(srq) => Self::OwnSrq(srq.into()),
        }
    }
}
impl Into<rc::ad::AEffectAffecteeFilter> for &CEffectTgtFilter {
    fn into(self) -> rc::ad::AEffectAffecteeFilter {
        match self {
            CEffectTgtFilter::Direct(dom) => rc::ad::AEffectAffecteeFilter::Direct(dom.into()),
            CEffectTgtFilter::Loc(dom) => rc::ad::AEffectAffecteeFilter::Loc(dom.into()),
            CEffectTgtFilter::LocGrp(dom, grp) => rc::ad::AEffectAffecteeFilter::LocGrp(dom.into(), *grp),
            CEffectTgtFilter::LocSrq(dom, srq) => rc::ad::AEffectAffecteeFilter::LocSrq(dom.into(), srq.into()),
            CEffectTgtFilter::OwnSrq(srq) => rc::ad::AEffectAffecteeFilter::OwnSrq(srq.into()),
        }
    }
}
