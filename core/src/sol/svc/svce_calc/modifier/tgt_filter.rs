use crate::{
    ad,
    defs::{EItemGrpId, EItemId},
    sol::{item::SolItem, svc::svce_calc::SolDomain},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) enum SolAffecteeFilter {
    Direct(SolDomain),
    Loc(SolDomain),
    LocGrp(SolDomain, EItemGrpId),
    LocSrq(SolDomain, EItemId),
    OwnSrq(EItemId),
}
impl SolAffecteeFilter {
    pub(super) fn from_a_effect_tgt_filter(
        a_effect_tgt_filter: &ad::AEffectAffecteeFilter,
        sol_item: &SolItem,
    ) -> Self {
        match a_effect_tgt_filter {
            ad::AEffectAffecteeFilter::Direct(dom) => Self::Direct(dom.into()),
            ad::AEffectAffecteeFilter::Loc(dom) => Self::Loc(dom.into()),
            ad::AEffectAffecteeFilter::LocGrp(dom, grp_id) => Self::LocGrp(dom.into(), *grp_id),
            ad::AEffectAffecteeFilter::LocSrq(dom, mod_srq) => Self::LocSrq(dom.into(), get_srq(mod_srq, sol_item)),
            ad::AEffectAffecteeFilter::OwnSrq(mod_srq) => Self::OwnSrq(get_srq(mod_srq, sol_item)),
        }
    }
    pub(super) fn from_a_buff_tgt_filter(
        a_buff_tgt_filter: &ad::ABuffTgtFilter,
        sol_domain: SolDomain,
        sol_item: &SolItem,
    ) -> Self {
        match a_buff_tgt_filter {
            ad::ABuffTgtFilter::Direct => Self::Direct(sol_domain),
            ad::ABuffTgtFilter::Loc => Self::Loc(sol_domain),
            ad::ABuffTgtFilter::LocGrp(grp_id) => Self::LocGrp(sol_domain, *grp_id),
            ad::ABuffTgtFilter::LocSrq(mod_srq) => Self::LocSrq(sol_domain, get_srq(mod_srq, sol_item)),
        }
    }
}

fn get_srq(mod_srq: &ad::AModSrq, sol_item: &SolItem) -> EItemId {
    match mod_srq {
        ad::AModSrq::SelfRef => sol_item.get_a_item_id(),
        ad::AModSrq::ItemId(item_id) => *item_id,
    }
}
