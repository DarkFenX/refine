use crate::handler_json::data::CModSrq;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CBuffTgtFilter {
    Direct,
    Loc,
    LocGrp(rc::EItemGrpId),
    LocSrq(CModSrq),
}
impl From<&rc::ad::ABuffTgtFilter> for CBuffTgtFilter {
    fn from(buff_tgt_filter: &rc::ad::ABuffTgtFilter) -> Self {
        match buff_tgt_filter {
            rc::ad::ABuffTgtFilter::Direct => Self::Direct,
            rc::ad::ABuffTgtFilter::Loc => Self::Loc,
            rc::ad::ABuffTgtFilter::LocGrp(grp) => Self::LocGrp(*grp),
            rc::ad::ABuffTgtFilter::LocSrq(srq) => Self::LocSrq(srq.into()),
        }
    }
}
impl Into<rc::ad::ABuffTgtFilter> for &CBuffTgtFilter {
    fn into(self) -> rc::ad::ABuffTgtFilter {
        match self {
            CBuffTgtFilter::Direct => rc::ad::ABuffTgtFilter::Direct,
            CBuffTgtFilter::Loc => rc::ad::ABuffTgtFilter::Loc,
            CBuffTgtFilter::LocGrp(grp) => rc::ad::ABuffTgtFilter::LocGrp(*grp),
            CBuffTgtFilter::LocSrq(srq) => rc::ad::ABuffTgtFilter::LocSrq(srq.into()),
        }
    }
}
