use super::ModDomain;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum ModAfeeFilter {
    Direct(ModDomain),
    Loc(ModDomain),
    LocGrp(ModDomain, rc::ReeInt),
    LocSrq(ModDomain, rc::ReeInt),
    OwnSrq(ModDomain, rc::ReeInt),
}
impl From<&rc::consts::ModAfeeFilter> for ModAfeeFilter {
    fn from(value: &rc::consts::ModAfeeFilter) -> Self {
        match value {
            rc::consts::ModAfeeFilter::Direct(dom) => Self::Direct(dom.into()),
            rc::consts::ModAfeeFilter::Loc(dom) => Self::Loc(dom.into()),
            rc::consts::ModAfeeFilter::LocGrp(dom, grp) => Self::LocGrp(dom.into(), *grp),
            rc::consts::ModAfeeFilter::LocSrq(dom, srq) => Self::LocSrq(dom.into(), *srq),
            rc::consts::ModAfeeFilter::OwnSrq(dom, srq) => Self::OwnSrq(dom.into(), *srq),
        }
    }
}
impl Into<rc::consts::ModAfeeFilter> for &ModAfeeFilter {
    fn into(self) -> rc::consts::ModAfeeFilter {
        match self {
            ModAfeeFilter::Direct(dom) => rc::consts::ModAfeeFilter::Direct(dom.into()),
            ModAfeeFilter::Loc(dom) => rc::consts::ModAfeeFilter::Loc(dom.into()),
            ModAfeeFilter::LocGrp(dom, grp) => rc::consts::ModAfeeFilter::LocGrp(dom.into(), *grp),
            ModAfeeFilter::LocSrq(dom, srq) => rc::consts::ModAfeeFilter::LocSrq(dom.into(), *srq),
            ModAfeeFilter::OwnSrq(dom, srq) => rc::consts::ModAfeeFilter::OwnSrq(dom.into(), *srq),
        }
    }
}
