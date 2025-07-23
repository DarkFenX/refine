use crate::cacher_json::data::{CItemGrpId, CModifierSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cacher_json) enum CBuffAffecteeFilter {
    Direct,
    Loc,
    LocGrp(CItemGrpId),
    LocSrq(CModifierSrq),
}
impl From<&rc::ad::ABuffAffecteeFilter> for CBuffAffecteeFilter {
    fn from(a_buff_affectee_filter: &rc::ad::ABuffAffecteeFilter) -> Self {
        match a_buff_affectee_filter {
            rc::ad::ABuffAffecteeFilter::Direct => Self::Direct,
            rc::ad::ABuffAffecteeFilter::Loc => Self::Loc,
            rc::ad::ABuffAffecteeFilter::LocGrp(grp) => Self::LocGrp(*grp),
            rc::ad::ABuffAffecteeFilter::LocSrq(srq) => Self::LocSrq(srq.into()),
        }
    }
}
impl From<&CBuffAffecteeFilter> for rc::ad::ABuffAffecteeFilter {
    fn from(c_buff_affectee_filter: &CBuffAffecteeFilter) -> Self {
        match c_buff_affectee_filter {
            CBuffAffecteeFilter::Direct => Self::Direct,
            CBuffAffecteeFilter::Loc => Self::Loc,
            CBuffAffecteeFilter::LocGrp(grp) => Self::LocGrp(*grp),
            CBuffAffecteeFilter::LocSrq(srq) => Self::LocSrq(srq.into()),
        }
    }
}
