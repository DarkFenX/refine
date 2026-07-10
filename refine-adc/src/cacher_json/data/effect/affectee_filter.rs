use super::location::CEffectLocation;
use crate::cacher_json::data::{AdaptedConv, CModifierSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum CEffectAffecteeFilter {
    Direct(CEffectLocation),
    Loc(CEffectLocation),
    LocGrp(CEffectLocation, i32),
    LocSrq(CEffectLocation, CModifierSrq),
    OwnSrq(CModifierSrq),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CEffectAffecteeFilter {
    type AEntity = rc::ad::AEffectAffecteeFilter;

    fn from_adapted(a_effect_affectee_filter: &Self::AEntity) -> Self {
        match a_effect_affectee_filter {
            Self::AEntity::Direct(loc) => Self::Direct(CEffectLocation::from_adapted(loc)),
            Self::AEntity::Loc(loc) => Self::Loc(CEffectLocation::from_adapted(loc)),
            Self::AEntity::LocGrp(loc, grp) => Self::LocGrp(CEffectLocation::from_adapted(loc), grp.into_i32()),
            Self::AEntity::LocSrq(loc, srq) => {
                Self::LocSrq(CEffectLocation::from_adapted(loc), CModifierSrq::from_adapted(srq))
            }
            Self::AEntity::OwnSrq(srq) => Self::OwnSrq(CModifierSrq::from_adapted(srq)),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Direct(loc) => Self::AEntity::Direct(loc.into_adapted()),
            Self::Loc(loc) => Self::AEntity::Loc(loc.into_adapted()),
            Self::LocGrp(loc, grp) => Self::AEntity::LocGrp(loc.into_adapted(), rc::ad::AItemGrpId::from_i32(grp)),
            Self::LocSrq(loc, srq) => Self::AEntity::LocSrq(loc.into_adapted(), srq.into_adapted()),
            Self::OwnSrq(srq) => Self::AEntity::OwnSrq(srq.into_adapted()),
        }
    }
}
