use crate::{
    ad,
    defs::{EItemGrpId, EItemId},
    ss::item::SsItem,
};

use super::SsModDomain;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) enum SsModTgtFilter {
    Direct(SsModDomain),
    Loc(SsModDomain),
    LocGrp(SsModDomain, EItemGrpId),
    LocSrq(SsModDomain, EItemId),
    OwnSrq(EItemId),
}
impl SsModTgtFilter {
    pub(super) fn from_a_effect_tgt_filter(a_effect_tgt_filter: &ad::AEffectTgtFilter, ss_item: &SsItem) -> Self {
        match a_effect_tgt_filter {
            ad::AEffectTgtFilter::Direct(dom) => Self::Direct(dom.into()),
            ad::AEffectTgtFilter::Loc(dom) => Self::Loc(dom.into()),
            ad::AEffectTgtFilter::LocGrp(dom, grp_id) => Self::LocGrp(dom.into(), *grp_id),
            ad::AEffectTgtFilter::LocSrq(dom, mod_srq) => Self::LocSrq(dom.into(), get_srq(mod_srq, ss_item)),
            ad::AEffectTgtFilter::OwnSrq(mod_srq) => Self::OwnSrq(get_srq(mod_srq, ss_item)),
        }
    }
    pub(super) fn from_a_buff_tgt_filter(
        a_buff_tgt_filter: &ad::ABuffTgtFilter,
        ss_domain: SsModDomain,
        ss_item: &SsItem,
    ) -> Self {
        match a_buff_tgt_filter {
            ad::ABuffTgtFilter::Direct => Self::Direct(ss_domain),
            ad::ABuffTgtFilter::Loc => Self::Loc(ss_domain),
            ad::ABuffTgtFilter::LocGrp(grp_id) => Self::LocGrp(ss_domain, *grp_id),
            ad::ABuffTgtFilter::LocSrq(mod_srq) => Self::LocSrq(ss_domain, get_srq(mod_srq, ss_item)),
        }
    }
}

fn get_srq(mod_srq: &ad::AModSrq, ss_item: &SsItem) -> EItemId {
    match mod_srq {
        ad::AModSrq::SelfRef => ss_item.get_a_item_id(),
        ad::AModSrq::ItemId(item_id) => *item_id,
    }
}
