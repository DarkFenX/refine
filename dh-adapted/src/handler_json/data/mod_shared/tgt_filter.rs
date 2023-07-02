use crate::handler_json::data::{CModDomain, CModSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModTgtFilter {
    Direct(CModDomain),
    Loc(CModDomain),
    LocGrp(CModDomain, rc::EItemGrpId),
    LocSrq(CModDomain, CModSrq),
    OwnSrq(CModDomain, CModSrq),
}
impl From<&rc::ad::AModTgtFilter> for CModTgtFilter {
    fn from(mod_tgt_filter: &rc::ad::AModTgtFilter) -> Self {
        match mod_tgt_filter {
            rc::ad::AModTgtFilter::Direct(dom) => Self::Direct(dom.into()),
            rc::ad::AModTgtFilter::Loc(dom) => Self::Loc(dom.into()),
            rc::ad::AModTgtFilter::LocGrp(dom, grp) => Self::LocGrp(dom.into(), *grp),
            rc::ad::AModTgtFilter::LocSrq(dom, srq) => Self::LocSrq(dom.into(), srq.into()),
            rc::ad::AModTgtFilter::OwnSrq(dom, srq) => Self::OwnSrq(dom.into(), srq.into()),
        }
    }
}
impl Into<rc::ad::AModTgtFilter> for &CModTgtFilter {
    fn into(self) -> rc::ad::AModTgtFilter {
        match self {
            CModTgtFilter::Direct(dom) => rc::ad::AModTgtFilter::Direct(dom.into()),
            CModTgtFilter::Loc(dom) => rc::ad::AModTgtFilter::Loc(dom.into()),
            CModTgtFilter::LocGrp(dom, grp) => rc::ad::AModTgtFilter::LocGrp(dom.into(), *grp),
            CModTgtFilter::LocSrq(dom, srq) => rc::ad::AModTgtFilter::LocSrq(dom.into(), srq.into()),
            CModTgtFilter::OwnSrq(dom, srq) => rc::ad::AModTgtFilter::OwnSrq(dom.into(), srq.into()),
        }
    }
}
