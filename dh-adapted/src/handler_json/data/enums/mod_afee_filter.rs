use super::CModDomain;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModAfeeFilter {
    Direct(CModDomain),
    Loc(CModDomain),
    LocGrp(CModDomain, rc::EItemGrpId),
    LocSrq(CModDomain, rc::EItemId),
    OwnSrq(CModDomain, rc::EItemId),
}
impl From<&rc::consts::ModAfeeFilter> for CModAfeeFilter {
    fn from(mod_afee_filter: &rc::consts::ModAfeeFilter) -> Self {
        match mod_afee_filter {
            rc::consts::ModAfeeFilter::Direct(dom) => Self::Direct(dom.into()),
            rc::consts::ModAfeeFilter::Loc(dom) => Self::Loc(dom.into()),
            rc::consts::ModAfeeFilter::LocGrp(dom, grp) => Self::LocGrp(dom.into(), *grp),
            rc::consts::ModAfeeFilter::LocSrq(dom, srq) => Self::LocSrq(dom.into(), *srq),
            rc::consts::ModAfeeFilter::OwnSrq(dom, srq) => Self::OwnSrq(dom.into(), *srq),
        }
    }
}
impl Into<rc::consts::ModAfeeFilter> for &CModAfeeFilter {
    fn into(self) -> rc::consts::ModAfeeFilter {
        match self {
            CModAfeeFilter::Direct(dom) => rc::consts::ModAfeeFilter::Direct(dom.into()),
            CModAfeeFilter::Loc(dom) => rc::consts::ModAfeeFilter::Loc(dom.into()),
            CModAfeeFilter::LocGrp(dom, grp) => rc::consts::ModAfeeFilter::LocGrp(dom.into(), *grp),
            CModAfeeFilter::LocSrq(dom, srq) => rc::consts::ModAfeeFilter::LocSrq(dom.into(), *srq),
            CModAfeeFilter::OwnSrq(dom, srq) => rc::consts::ModAfeeFilter::OwnSrq(dom.into(), *srq),
        }
    }
}
