use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, SsItemId},
    ss::{
        item::SsItem,
        svc::{
            svce_calc::{SsAffecteeFilter, SsModAggrMode, SsModDomain, SsModOp, SsModType},
            SsSvcs,
        },
        SsView,
    },
    util::Result,
};

use super::SsAttrModSrc;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::svce_calc) struct SsAttrMod {
    pub(in crate::ss::svc::svce_calc) mod_type: SsModType,
    pub(in crate::ss::svc::svce_calc) affector_item_id: SsItemId,
    // This field is here just for hash
    pub(in crate::ss::svc::svce_calc) effect_id: EEffectId,
    val_getter: SsAttrModSrc,
    pub(in crate::ss::svc::svce_calc) op: SsModOp,
    pub(in crate::ss::svc::svce_calc) aggr_mode: SsModAggrMode,
    pub(in crate::ss::svc::svce_calc) affectee_filter: SsAffecteeFilter,
    pub(in crate::ss::svc::svce_calc) affectee_attr_id: EAttrId,
}
impl SsAttrMod {
    pub(super) fn new(
        mod_type: SsModType,
        affector_item_id: SsItemId,
        effect_id: EEffectId,
        val_getter: SsAttrModSrc,
        op: SsModOp,
        aggr_mode: SsModAggrMode,
        affectee_filter: SsAffecteeFilter,
        affectee_attr_id: EAttrId,
    ) -> Self {
        Self {
            mod_type,
            affector_item_id,
            effect_id,
            val_getter,
            op,
            aggr_mode,
            affectee_filter,
            affectee_attr_id,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn from_a_effect(
        affector_item: &SsItem,
        a_effect: &ad::AEffect,
        a_mod: &ad::AEffectAttrMod,
        mod_type: SsModType,
    ) -> Self {
        Self::new(
            mod_type,
            affector_item.get_id(),
            a_effect.id,
            SsAttrModSrc::AttrId(a_mod.src_attr_id),
            (&a_mod.op).into(),
            SsModAggrMode::Stack,
            SsAffecteeFilter::from_a_effect_tgt_filter(&a_mod.tgt_filter, affector_item),
            a_mod.tgt_attr_id,
        )
    }
    pub(in crate::ss::svc::svce_calc) fn from_a_buff(
        affector_item: &SsItem,
        a_effect: &ad::AEffect,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffAttrMod,
        affector_attr_id: EAttrId,
        mod_type: SsModType,
        domain: SsModDomain,
    ) -> Self {
        Self::new(
            mod_type,
            affector_item.get_id(),
            a_effect.id,
            SsAttrModSrc::AttrId(affector_attr_id),
            (&a_buff.op).into(),
            SsModAggrMode::from_a_buff(a_buff),
            SsAffecteeFilter::from_a_buff_tgt_filter(&a_mod.tgt_filter, domain, affector_item),
            a_mod.tgt_attr_id,
        )
    }
    pub(in crate::ss::svc::svce_calc) fn get_src_attr_id(&self) -> Option<EAttrId> {
        self.val_getter.get_src_attr_id()
    }
    pub(in crate::ss::svc::svce_calc) fn get_srcs(&self, ss_view: &SsView) -> Vec<(SsItemId, EAttrId)> {
        self.val_getter.get_srcs(ss_view, &self.affector_item_id)
    }
    pub(in crate::ss::svc::svce_calc) fn get_mod_val(&self, svc: &mut SsSvcs, ss_view: &SsView) -> Result<AttrVal> {
        self.val_getter.get_mod_val(svc, ss_view, &self.affector_item_id)
    }
    pub(in crate::ss::svc::svce_calc) fn on_effect_stop(&self, svc: &mut SsSvcs, ss_view: &SsView) {
        self.val_getter.on_effect_stop(svc, ss_view, &self.affector_item_id)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::ss::svc::svce_calc) fn needs_revision_on_item_add(&self) -> bool {
        self.val_getter.revisable_on_item_add()
    }
    pub(in crate::ss::svc::svce_calc) fn needs_revision_on_item_remove(&self) -> bool {
        self.val_getter.revisable_on_item_remove()
    }
    pub(in crate::ss::svc::svce_calc) fn revise_on_item_add(&self, added_item: &SsItem, ss_view: &SsView) -> bool {
        let src_item = match ss_view.items.get_item(&self.affector_item_id) {
            Ok(item) => item,
            _ => return false,
        };
        self.val_getter.revise_on_item_add(src_item, added_item)
    }
    pub(in crate::ss::svc::svce_calc) fn revise_on_item_remove(&self, added_item: &SsItem, ss_view: &SsView) -> bool {
        let src_item = match ss_view.items.get_item(&self.affector_item_id) {
            Ok(item) => item,
            _ => return false,
        };
        self.val_getter.revise_on_item_remove(src_item, added_item)
    }
}
