use crate::{
    ad,
    defs::{EAttrId, EEffectId, SsItemId},
    shr::{ModAggrMode, ModOp},
    ss::item::SsItem,
};

use super::SsModTgtFilter;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
        src_a_effect: &ad::ArcEffect,
        src_a_mod: &ad::AEffectAttrMod,
    ) -> Self {
        Self::new(
            src_ss_item.get_id(),
            src_a_effect.id,
            src_a_mod.src_attr_id,
            SsModTgtFilter::from_a_mod_tgt_filter(&src_a_mod.tgt_filter, src_ss_item),
            src_a_mod.op,
            ModAggrMode::Stack,
            src_a_mod.tgt_attr_id,
        )
    }
}
