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
impl From<&rc::ad::ModAfeeFilter> for CModAfeeFilter {
    fn from(mod_afee_filter: &rc::ad::ModAfeeFilter) -> Self {
        match mod_afee_filter {
            rc::ad::ModAfeeFilter::Direct(dom) => Self::Direct(dom.into()),
            rc::ad::ModAfeeFilter::Loc(dom) => Self::Loc(dom.into()),
            rc::ad::ModAfeeFilter::LocGrp(dom, grp) => Self::LocGrp(dom.into(), *grp),
            rc::ad::ModAfeeFilter::LocSrq(dom, srq) => Self::LocSrq(dom.into(), srq.into()),
            rc::ad::ModAfeeFilter::OwnSrq(dom, srq) => Self::OwnSrq(dom.into(), srq.into()),
        }
    }
}
impl Into<rc::ad::ModAfeeFilter> for &CModAfeeFilter {
    fn into(self) -> rc::ad::ModAfeeFilter {
        match self {
            CModAfeeFilter::Direct(dom) => rc::ad::ModAfeeFilter::Direct(dom.into()),
            CModAfeeFilter::Loc(dom) => rc::ad::ModAfeeFilter::Loc(dom.into()),
            CModAfeeFilter::LocGrp(dom, grp) => rc::ad::ModAfeeFilter::LocGrp(dom.into(), *grp),
            CModAfeeFilter::LocSrq(dom, srq) => rc::ad::ModAfeeFilter::LocSrq(dom.into(), srq.into()),
            CModAfeeFilter::OwnSrq(dom, srq) => rc::ad::ModAfeeFilter::OwnSrq(dom.into(), srq.into()),
        }
    }
}
