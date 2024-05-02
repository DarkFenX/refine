use crate::handler_json::data::CModSrq;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CBuffAffecteeFilter {
    Direct,
    Loc,
    LocGrp(rc::EItemGrpId),
    LocSrq(CModSrq),
}
impl From<&rc::ad::ABuffAffecteeFilter> for CBuffAffecteeFilter {
    fn from(buff_affectee_filter: &rc::ad::ABuffAffecteeFilter) -> Self {
        match buff_affectee_filter {
            rc::ad::ABuffAffecteeFilter::Direct => Self::Direct,
            rc::ad::ABuffAffecteeFilter::Loc => Self::Loc,
            rc::ad::ABuffAffecteeFilter::LocGrp(grp) => Self::LocGrp(*grp),
            rc::ad::ABuffAffecteeFilter::LocSrq(srq) => Self::LocSrq(srq.into()),
        }
    }
}
impl Into<rc::ad::ABuffAffecteeFilter> for &CBuffAffecteeFilter {
    fn into(self) -> rc::ad::ABuffAffecteeFilter {
        match self {
            CBuffAffecteeFilter::Direct => rc::ad::ABuffAffecteeFilter::Direct,
            CBuffAffecteeFilter::Loc => rc::ad::ABuffAffecteeFilter::Loc,
            CBuffAffecteeFilter::LocGrp(grp) => rc::ad::ABuffAffecteeFilter::LocGrp(*grp),
            CBuffAffecteeFilter::LocSrq(srq) => rc::ad::ABuffAffecteeFilter::LocSrq(srq.into()),
        }
    }
}
