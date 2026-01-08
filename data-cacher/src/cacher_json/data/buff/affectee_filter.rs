use super::super::shared::CModifierSrq;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum CBuffAffecteeFilter {
    Direct,
    Loc,
    LocGrp(i32),
    LocSrq(CModifierSrq),
}
impl CBuffAffecteeFilter {
    pub(super) fn from_adapted(a_buff_affectee_filter: &rc::ad::ABuffAffecteeFilter) -> Self {
        match a_buff_affectee_filter {
            rc::ad::ABuffAffecteeFilter::Direct => Self::Direct,
            rc::ad::ABuffAffecteeFilter::Loc => Self::Loc,
            rc::ad::ABuffAffecteeFilter::LocGrp(grp) => Self::LocGrp(grp.into_i32()),
            rc::ad::ABuffAffecteeFilter::LocSrq(srq) => Self::LocSrq(CModifierSrq::from_adapted(srq)),
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::ABuffAffecteeFilter {
        match self {
            Self::Direct => rc::ad::ABuffAffecteeFilter::Direct,
            Self::Loc => rc::ad::ABuffAffecteeFilter::Loc,
            Self::LocGrp(grp) => rc::ad::ABuffAffecteeFilter::LocGrp(rc::ad::AItemGrpId::from_i32(grp)),
            Self::LocSrq(srq) => rc::ad::ABuffAffecteeFilter::LocSrq(srq.into_adapted()),
        }
    }
}
