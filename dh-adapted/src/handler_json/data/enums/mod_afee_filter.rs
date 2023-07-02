use super::{CModDomain, CModSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModAfeeFilter {
    Direct(CModDomain),
    Loc(CModDomain),
    LocGrp(CModDomain, rc::EItemGrpId),
    LocSrq(CModDomain, CModSrq),
    OwnSrq(CModDomain, CModSrq),
}
impl From<&rc::ad::AModTgtFilter> for CModAfeeFilter {
    fn from(mod_afee_filter: &rc::ad::AModTgtFilter) -> Self {
        match mod_afee_filter {
            rc::ad::AModTgtFilter::Direct(dom) => Self::Direct(dom.into()),
            rc::ad::AModTgtFilter::Loc(dom) => Self::Loc(dom.into()),
            rc::ad::AModTgtFilter::LocGrp(dom, grp) => Self::LocGrp(dom.into(), *grp),
            rc::ad::AModTgtFilter::LocSrq(dom, srq) => Self::LocSrq(dom.into(), srq.into()),
            rc::ad::AModTgtFilter::OwnSrq(dom, srq) => Self::OwnSrq(dom.into(), srq.into()),
        }
    }
}
impl Into<rc::ad::AModTgtFilter> for &CModAfeeFilter {
    fn into(self) -> rc::ad::AModTgtFilter {
        match self {
            CModAfeeFilter::Direct(dom) => rc::ad::AModTgtFilter::Direct(dom.into()),
            CModAfeeFilter::Loc(dom) => rc::ad::AModTgtFilter::Loc(dom.into()),
            CModAfeeFilter::LocGrp(dom, grp) => rc::ad::AModTgtFilter::LocGrp(dom.into(), *grp),
            CModAfeeFilter::LocSrq(dom, srq) => rc::ad::AModTgtFilter::LocSrq(dom.into(), srq.into()),
            CModAfeeFilter::OwnSrq(dom, srq) => rc::ad::AModTgtFilter::OwnSrq(dom.into(), srq.into()),
        }
    }
}
