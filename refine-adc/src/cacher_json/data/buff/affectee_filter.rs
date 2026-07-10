use crate::cacher_json::data::{AdaptedConv, CModifierSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum CBuffAffecteeFilter {
    Direct,
    Loc,
    LocGrp(i32),
    LocSrq(CModifierSrq),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CBuffAffecteeFilter {
    type AEntity = rc::ad::ABuffAffecteeFilter;

    fn from_adapted(a_buff_affectee_filter: &Self::AEntity) -> Self {
        match a_buff_affectee_filter {
            Self::AEntity::Direct => Self::Direct,
            Self::AEntity::Loc => Self::Loc,
            Self::AEntity::LocGrp(grp) => Self::LocGrp(grp.into_i32()),
            Self::AEntity::LocSrq(srq) => Self::LocSrq(CModifierSrq::from_adapted(srq)),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Direct => Self::AEntity::Direct,
            Self::Loc => Self::AEntity::Loc,
            Self::LocGrp(grp) => Self::AEntity::LocGrp(rc::ad::AItemGrpId::from_i32(grp)),
            Self::LocSrq(srq) => Self::AEntity::LocSrq(srq.into_adapted()),
        }
    }
}
