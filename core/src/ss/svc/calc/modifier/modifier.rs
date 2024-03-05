use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, SsItemId},
    shr::{ModAggrMode, ModOp},
    ss::{item::SsItem, svc::SsSvcs, SsView},
    util::Result,
};

use super::{mod_src::SsAttrModSrc, SsModTgtFilter};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::calc) struct SsAttrMod {
    pub(in crate::ss::svc::calc) src_item_id: SsItemId,
    // This field is here just for hash
    pub(in crate::ss::svc::calc) src_effect_id: EEffectId,
    pub(in crate::ss::svc::calc) src_val_getter: SsAttrModSrc,
    pub(in crate::ss::svc::calc) op: ModOp,
    pub(in crate::ss::svc::calc) aggr_mode: ModAggrMode,
    pub(in crate::ss::svc::calc) tgt_filter: SsModTgtFilter,
    pub(in crate::ss::svc::calc) tgt_attr_id: EAttrId,
}
impl SsAttrMod {
    fn new(
        src_item_id: SsItemId,
        src_effect_id: EEffectId,
        src_val_getter: SsAttrModSrc,
        op: ModOp,
        aggr_mode: ModAggrMode,
        tgt_filter: SsModTgtFilter,
        tgt_attr_id: EAttrId,
    ) -> Self {
        Self {
            src_item_id,
            src_effect_id,
            src_val_getter,
            op,
            aggr_mode,
            tgt_filter,
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
            SsAttrModSrc::AttrId(src_a_mod.src_attr_id),
            src_a_mod.op,
            ModAggrMode::Stack,
            SsModTgtFilter::from_a_mod_tgt_filter(&src_a_mod.tgt_filter, src_ss_item),
            src_a_mod.tgt_attr_id,
        )
    }
    pub(in crate::ss::svc::calc) fn get_src_attr_id(&self) -> Option<EAttrId> {
        self.src_val_getter.get_src_attr_id()
    }
    pub(in crate::ss::svc::calc) fn get_mod_val(&self, svc: &mut SsSvcs, ss_view: &SsView) -> Result<AttrVal> {
        self.src_val_getter.get_mod_val(svc, ss_view, &self.src_item_id)
    }
}
