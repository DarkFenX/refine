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
    pub(super) fn from_a_mod_tgt_filter(a_mod_tgt_filter: &ad::AModTgtFilter, ss_item: &SsItem) -> Self {
        match a_mod_tgt_filter {
            ad::AModTgtFilter::Direct(dom) => Self::Direct(dom.into()),
            ad::AModTgtFilter::Loc(dom) => Self::Loc(dom.into()),
            ad::AModTgtFilter::LocGrp(dom, grp_id) => Self::LocGrp(dom.into(), *grp_id),
            ad::AModTgtFilter::LocSrq(dom, mod_srq) => Self::LocSrq(dom.into(), get_srq(mod_srq, ss_item)),
            ad::AModTgtFilter::OwnSrq(mod_srq) => Self::OwnSrq(get_srq(mod_srq, ss_item)),
        }
    }
}

fn get_srq(mod_srq: &ad::AModSrq, ss_item: &SsItem) -> EItemId {
    match mod_srq {
        ad::AModSrq::SelfRef => ss_item.get_a_item_id(),
        ad::AModSrq::ItemId(item_id) => *item_id,
    }
}
