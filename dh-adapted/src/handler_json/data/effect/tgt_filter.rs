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
impl From<&rc::ad::AEffectTgtFilter> for CEffectTgtFilter {
    fn from(effect_tgt_filter: &rc::ad::AEffectTgtFilter) -> Self {
        match effect_tgt_filter {
            rc::ad::AEffectTgtFilter::Direct(dom) => Self::Direct(dom.into()),
            rc::ad::AEffectTgtFilter::Loc(dom) => Self::Loc(dom.into()),
            rc::ad::AEffectTgtFilter::LocGrp(dom, grp) => Self::LocGrp(dom.into(), *grp),
            rc::ad::AEffectTgtFilter::LocSrq(dom, srq) => Self::LocSrq(dom.into(), srq.into()),
            rc::ad::AEffectTgtFilter::OwnSrq(srq) => Self::OwnSrq(srq.into()),
        }
    }
}
impl Into<rc::ad::AEffectTgtFilter> for &CEffectTgtFilter {
    fn into(self) -> rc::ad::AEffectTgtFilter {
        match self {
            CEffectTgtFilter::Direct(dom) => rc::ad::AEffectTgtFilter::Direct(dom.into()),
            CEffectTgtFilter::Loc(dom) => rc::ad::AEffectTgtFilter::Loc(dom.into()),
            CEffectTgtFilter::LocGrp(dom, grp) => rc::ad::AEffectTgtFilter::LocGrp(dom.into(), *grp),
            CEffectTgtFilter::LocSrq(dom, srq) => rc::ad::AEffectTgtFilter::LocSrq(dom.into(), srq.into()),
            CEffectTgtFilter::OwnSrq(srq) => rc::ad::AEffectTgtFilter::OwnSrq(srq.into()),
        }
    }
}
