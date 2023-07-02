use crate::{
    ad::{AAttrMod, ArcEffect},
    consts::{ModAfeeFilter, ModSrq},
    defs::{EAttrId, EEffectId, EItemGrpId, EItemId, SsItemId},
    shr::{ModAggrMode, ModDomain, ModOp},
    ss::item::SsItem,
};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub(in crate::ss::svc::calc) struct SsAttrMod {
    pub(in crate::ss::svc::calc) src_item_id: SsItemId,
    pub(in crate::ss::svc::calc) src_effect_id: EEffectId,
    pub(in crate::ss::svc::calc) src_attr_id: EAttrId,
    pub(in crate::ss::svc::calc) tgt_filter: SsModTgtFilter,
    pub(in crate::ss::svc::calc) op: ModOp,
    pub(in crate::ss::svc::calc) aggr_mode: ModAggrMode,
    pub(in crate::ss::svc::calc) tgt_attr_id: EAttrId,
}
impl SsAttrMod {
    fn new(
        src_item_id: SsItemId,
        src_effect_id: EEffectId,
        src_attr_id: EAttrId,
        tgt_filter: SsModTgtFilter,
        op: ModOp,
        aggr_mode: ModAggrMode,
        tgt_attr_id: EAttrId,
    ) -> Self {
        Self {
            src_item_id,
            src_effect_id,
            src_attr_id,
            tgt_filter,
            op,
            aggr_mode,
            tgt_attr_id,
        }
    }
    pub(in crate::ss::svc::calc) fn from_a_data(
        src_ss_item: &SsItem,
        src_a_effect: &ArcEffect,
        src_a_mod: &AAttrMod,
    ) -> Self {
        Self::new(
            src_ss_item.get_id(),
            src_a_effect.id,
            src_a_mod.afor_attr_id,
            SsModTgtFilter::from_a_mod_tgt_filter(&src_a_mod.afee_filter, src_ss_item),
            src_a_mod.op,
            ModAggrMode::Stack,
            src_a_mod.afee_attr_id,
        )
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub(in crate::ss::svc::calc) enum SsModTgtFilter {
    Direct(ModDomain),
    Loc(ModDomain),
    LocGrp(ModDomain, EItemGrpId),
    LocSrq(ModDomain, EItemId),
    OwnSrq(ModDomain, EItemId),
}
impl SsModTgtFilter {
    fn from_a_mod_tgt_filter(a_mod_tgt_filter: &ModAfeeFilter, ss_item: &SsItem) -> Self {
        match a_mod_tgt_filter {
            ModAfeeFilter::Direct(dom) => Self::Direct(*dom),
            ModAfeeFilter::Loc(dom) => Self::Loc(*dom),
            ModAfeeFilter::LocGrp(domain, grp_id) => Self::LocGrp(*domain, *grp_id),
            ModAfeeFilter::LocSrq(domain, mod_srq) => Self::LocSrq(*domain, get_srq(mod_srq, ss_item)),
            ModAfeeFilter::OwnSrq(domain, mod_srq) => Self::OwnSrq(*domain, get_srq(mod_srq, ss_item)),
        }
    }
}

fn get_srq(mod_srq: &ModSrq, ss_item: &SsItem) -> EItemId {
    match mod_srq {
        ModSrq::SelfRef => ss_item.get_a_item_id(),
        ModSrq::ItemId(item_id) => *item_id,
    }
}
