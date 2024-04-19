use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, SsItemId},
    ss::{
        item::SsItem,
        svc::{
            svce_calc::{modifier::SsModDomain, SsModAggrMode, SsModOp},
            SsSvcs,
        },
        SsView,
    },
    util::Result,
};

use super::{mod_src::SsAttrModSrc, SsModTgtFilter, SsModType};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) struct SsAttrMod {
    pub(in crate::ss::svc::svce_calc) mod_type: SsModType,
    pub(in crate::ss::svc::svce_calc) src_item_id: SsItemId,
    // This field is here just for hash
    pub(in crate::ss::svc::svce_calc) src_effect_id: EEffectId,
    src_val_getter: SsAttrModSrc,
    pub(in crate::ss::svc::svce_calc) op: SsModOp,
    pub(in crate::ss::svc::svce_calc) aggr_mode: SsModAggrMode,
    pub(in crate::ss::svc::svce_calc) tgt_filter: SsModTgtFilter,
    pub(in crate::ss::svc::svce_calc) tgt_attr_id: EAttrId,
}
impl SsAttrMod {
    pub(super) fn new(
        mod_type: SsModType,
        src_item_id: SsItemId,
        src_effect_id: EEffectId,
        src_val_getter: SsAttrModSrc,
        op: SsModOp,
        aggr_mode: SsModAggrMode,
        tgt_filter: SsModTgtFilter,
        tgt_attr_id: EAttrId,
    ) -> Self {
        Self {
            mod_type,
            src_item_id,
            src_effect_id,
            src_val_getter,
            op,
            aggr_mode,
            tgt_filter,
            tgt_attr_id,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn from_a_effect(
        src_ss_item: &SsItem,
        src_a_effect: &ad::AEffect,
        src_a_mod: &ad::AEffectAttrMod,
        mod_type: SsModType,
    ) -> Self {
        Self::new(
            mod_type,
            src_ss_item.get_id(),
            src_a_effect.id,
            SsAttrModSrc::AttrId(src_a_mod.src_attr_id),
            (&src_a_mod.op).into(),
            SsModAggrMode::Stack,
            SsModTgtFilter::from_a_effect_tgt_filter(&src_a_mod.tgt_filter, src_ss_item),
            src_a_mod.tgt_attr_id,
        )
    }
    pub(in crate::ss::svc::svce_calc) fn from_a_buff(
        src_ss_item: &SsItem,
        src_a_effect: &ad::AEffect,
        src_a_buff: &ad::ABuff,
        src_a_mod: &ad::ABuffAttrMod,
        src_attr_id: EAttrId,
        mod_type: SsModType,
        ss_domain: SsModDomain,
    ) -> Self {
        Self::new(
            mod_type,
            src_ss_item.get_id(),
            src_a_effect.id,
            SsAttrModSrc::AttrId(src_attr_id),
            (&src_a_buff.op).into(),
            SsModAggrMode::from_a_buff(src_a_buff),
            SsModTgtFilter::from_a_buff_tgt_filter(&src_a_mod.tgt_filter, ss_domain, src_ss_item),
            src_a_mod.tgt_attr_id,
        )
    }
    pub(in crate::ss::svc::svce_calc) fn get_src_attr_id(&self) -> Option<EAttrId> {
        self.src_val_getter.get_src_attr_id()
    }
    pub(in crate::ss::svc::svce_calc) fn get_srcs(&self, ss_view: &SsView) -> Vec<(SsItemId, EAttrId)> {
        self.src_val_getter.get_srcs(ss_view, &self.src_item_id)
    }
    pub(in crate::ss::svc::svce_calc) fn get_mod_val(&self, svc: &mut SsSvcs, ss_view: &SsView) -> Result<AttrVal> {
        self.src_val_getter.get_mod_val(svc, ss_view, &self.src_item_id)
    }
    pub(in crate::ss::svc::svce_calc) fn on_effect_stop(&self, svc: &mut SsSvcs, ss_view: &SsView) {
        self.src_val_getter.on_effect_stop(svc, ss_view, &self.src_item_id)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::ss::svc::svce_calc) fn needs_revision_on_item_add(&self) -> bool {
        self.src_val_getter.revisable_on_item_add()
    }
    pub(in crate::ss::svc::svce_calc) fn needs_revision_on_item_remove(&self) -> bool {
        self.src_val_getter.revisable_on_item_remove()
    }
    pub(in crate::ss::svc::svce_calc) fn revise_on_item_add(&self, added_item: &SsItem, ss_view: &SsView) -> bool {
        let src_item = match ss_view.items.get_item(&self.src_item_id) {
            Ok(item) => item,
            _ => return false,
        };
        self.src_val_getter.revise_on_item_add(src_item, added_item)
    }
    pub(in crate::ss::svc::svce_calc) fn revise_on_item_remove(&self, added_item: &SsItem, ss_view: &SsView) -> bool {
        let src_item = match ss_view.items.get_item(&self.src_item_id) {
            Ok(item) => item,
            _ => return false,
        };
        self.src_val_getter.revise_on_item_remove(src_item, added_item)
    }
}
