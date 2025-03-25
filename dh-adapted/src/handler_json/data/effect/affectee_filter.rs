use crate::handler_json::data::{CEffectLocation, CItemGrpId, CModifierSrq};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CEffectAffecteeFilter {
    Direct(CEffectLocation),
    Loc(CEffectLocation),
    LocGrp(CEffectLocation, CItemGrpId),
    LocSrq(CEffectLocation, CModifierSrq),
    OwnSrq(CModifierSrq),
}
impl From<&rc::ad::AEffectAffecteeFilter> for CEffectAffecteeFilter {
    fn from(a_effect_affectee_filter: &rc::ad::AEffectAffecteeFilter) -> Self {
        match a_effect_affectee_filter {
            rc::ad::AEffectAffecteeFilter::Direct(loc) => Self::Direct(loc.into()),
            rc::ad::AEffectAffecteeFilter::Loc(loc) => Self::Loc(loc.into()),
            rc::ad::AEffectAffecteeFilter::LocGrp(loc, grp) => Self::LocGrp(loc.into(), *grp),
            rc::ad::AEffectAffecteeFilter::LocSrq(loc, srq) => Self::LocSrq(loc.into(), srq.into()),
            rc::ad::AEffectAffecteeFilter::OwnSrq(srq) => Self::OwnSrq(srq.into()),
        }
    }
}
impl From<&CEffectAffecteeFilter> for rc::ad::AEffectAffecteeFilter {
    fn from(c_effect_affectee_filter: &CEffectAffecteeFilter) -> Self {
        match c_effect_affectee_filter {
            CEffectAffecteeFilter::Direct(loc) => Self::Direct(loc.into()),
            CEffectAffecteeFilter::Loc(loc) => Self::Loc(loc.into()),
            CEffectAffecteeFilter::LocGrp(loc, grp) => Self::LocGrp(loc.into(), *grp),
            CEffectAffecteeFilter::LocSrq(loc, srq) => Self::LocSrq(loc.into(), srq.into()),
            CEffectAffecteeFilter::OwnSrq(srq) => Self::OwnSrq(srq.into()),
        }
    }
}
