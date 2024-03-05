use crate::{
    ad,
    defs::{EItemGrpId, EItemId},
    shr::ModDomain,
    ss::item::SsItem,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::calc) enum SsModTgtFilter {
    Direct(ModDomain),
    Loc(ModDomain),
    LocGrp(ModDomain, EItemGrpId),
    LocSrq(ModDomain, EItemId),
    OwnSrq(EItemId),
}
impl SsModTgtFilter {
    pub(super) fn from_a_mod_tgt_filter(a_mod_tgt_filter: &ad::AModTgtFilter, ss_item: &SsItem) -> Self {
        match a_mod_tgt_filter {
            ad::AModTgtFilter::Direct(dom) => Self::Direct(*dom),
            ad::AModTgtFilter::Loc(dom) => Self::Loc(*dom),
            ad::AModTgtFilter::LocGrp(domain, grp_id) => Self::LocGrp(*domain, *grp_id),
            ad::AModTgtFilter::LocSrq(domain, mod_srq) => Self::LocSrq(*domain, get_srq(mod_srq, ss_item)),
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
