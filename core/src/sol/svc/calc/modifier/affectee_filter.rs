use crate::{
    ad,
    defs::{EItemGrpId, EItemId},
    sol::{svc::calc::SolLocation, uad::item::SolItem},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) enum SolAffecteeFilter {
    Direct(SolLocation),
    Loc(SolLocation),
    LocGrp(SolLocation, EItemGrpId),
    LocSrq(SolLocation, EItemId),
    OwnSrq(EItemId),
}
impl SolAffecteeFilter {
    pub(super) fn from_a_effect_affectee_filter(
        a_effect_affectee_filter: &ad::AEffectAffecteeFilter,
        sol_item: &SolItem,
    ) -> Self {
        match a_effect_affectee_filter {
            ad::AEffectAffecteeFilter::Direct(loc) => Self::Direct(loc.into()),
            ad::AEffectAffecteeFilter::Loc(loc) => Self::Loc(loc.into()),
            ad::AEffectAffecteeFilter::LocGrp(loc, grp_id) => Self::LocGrp(loc.into(), *grp_id),
            ad::AEffectAffecteeFilter::LocSrq(loc, mod_srq) => Self::LocSrq(loc.into(), get_srq(mod_srq, sol_item)),
            ad::AEffectAffecteeFilter::OwnSrq(mod_srq) => Self::OwnSrq(get_srq(mod_srq, sol_item)),
        }
    }
    pub(super) fn from_a_buff_affectee_filter(
        a_buff_affectee_filter: &ad::ABuffAffecteeFilter,
        sol_loc: SolLocation,
        sol_item: &SolItem,
    ) -> Self {
        match a_buff_affectee_filter {
            ad::ABuffAffecteeFilter::Direct => Self::Direct(sol_loc),
            ad::ABuffAffecteeFilter::Loc => Self::Loc(sol_loc),
            ad::ABuffAffecteeFilter::LocGrp(grp_id) => Self::LocGrp(sol_loc, *grp_id),
            ad::ABuffAffecteeFilter::LocSrq(mod_srq) => Self::LocSrq(sol_loc, get_srq(mod_srq, sol_item)),
        }
    }
}

fn get_srq(mod_srq: &ad::AModifierSrq, sol_item: &SolItem) -> EItemId {
    match mod_srq {
        ad::AModifierSrq::SelfRef => sol_item.get_type_id(),
        ad::AModifierSrq::ItemId(item_id) => *item_id,
    }
}
