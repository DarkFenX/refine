use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, SolItemId},
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{SolAffecteeFilter, SolModAggrMode, SolModDomain, SolModOp, SolModType},
            SolSvcs,
        },
        SolView,
    },
    util::Result,
};

use super::SolAttrModSrc;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) struct SolAttrMod {
    pub(in crate::sol::svc::svce_calc) mod_type: SolModType,
    pub(in crate::sol::svc::svce_calc) affector_item_id: SolItemId,
    // This field is here just for hash
    pub(in crate::sol::svc::svce_calc) effect_id: EEffectId,
    val_getter: SolAttrModSrc,
    pub(in crate::sol::svc::svce_calc) op: SolModOp,
    pub(in crate::sol::svc::svce_calc) aggr_mode: SolModAggrMode,
    pub(in crate::sol::svc::svce_calc) affectee_filter: SolAffecteeFilter,
    pub(in crate::sol::svc::svce_calc) affectee_attr_id: EAttrId,
}
impl SolAttrMod {
    pub(super) fn new(
        mod_type: SolModType,
        affector_item_id: SolItemId,
        effect_id: EEffectId,
        val_getter: SolAttrModSrc,
        op: SolModOp,
        aggr_mode: SolModAggrMode,
        affectee_filter: SolAffecteeFilter,
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
    pub(in crate::sol::svc::svce_calc) fn from_a_effect(
        affector_item: &SolItem,
        a_effect: &ad::AEffect,
        a_mod: &ad::AEffectAttrMod,
        mod_type: SolModType,
    ) -> Self {
        Self::new(
            mod_type,
            affector_item.get_id(),
            a_effect.id,
            SolAttrModSrc::AttrId(a_mod.src_attr_id),
            (&a_mod.op).into(),
            SolModAggrMode::Stack,
            SolAffecteeFilter::from_a_effect_tgt_filter(&a_mod.tgt_filter, affector_item),
            a_mod.tgt_attr_id,
        )
    }
    pub(in crate::sol::svc::svce_calc) fn from_a_buff(
        affector_item: &SolItem,
        a_effect: &ad::AEffect,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffAttrMod,
        affector_attr_id: EAttrId,
        mod_type: SolModType,
        domain: SolModDomain,
    ) -> Self {
        Self::new(
            mod_type,
            affector_item.get_id(),
            a_effect.id,
            SolAttrModSrc::AttrId(affector_attr_id),
            (&a_buff.op).into(),
            SolModAggrMode::from_a_buff(a_buff),
            SolAffecteeFilter::from_a_buff_tgt_filter(&a_mod.tgt_filter, domain, affector_item),
            a_mod.tgt_attr_id,
        )
    }
    pub(in crate::sol::svc::svce_calc) fn get_src_attr_id(&self) -> Option<EAttrId> {
        self.val_getter.get_src_attr_id()
    }
    pub(in crate::sol::svc::svce_calc) fn get_srcs(&self, sol_view: &SolView) -> Vec<(SolItemId, EAttrId)> {
        self.val_getter.get_srcs(sol_view, &self.affector_item_id)
    }
    pub(in crate::sol::svc::svce_calc) fn get_mod_val(&self, svc: &mut SolSvcs, sol_view: &SolView) -> Result<AttrVal> {
        self.val_getter.get_mod_val(svc, sol_view, &self.affector_item_id)
    }
    pub(in crate::sol::svc::svce_calc) fn on_effect_stop(&self, svc: &mut SolSvcs, sol_view: &SolView) {
        self.val_getter.on_effect_stop(svc, sol_view, &self.affector_item_id)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::sol::svc::svce_calc) fn needs_revision_on_item_add(&self) -> bool {
        self.val_getter.revisable_on_item_add()
    }
    pub(in crate::sol::svc::svce_calc) fn needs_revision_on_item_remove(&self) -> bool {
        self.val_getter.revisable_on_item_remove()
    }
    pub(in crate::sol::svc::svce_calc) fn revise_on_item_add(&self, added_item: &SolItem, sol_view: &SolView) -> bool {
        let src_item = sol_view.items.get_item(&self.affector_item_id).unwrap();
        self.val_getter.revise_on_item_add(src_item, added_item)
    }
    pub(in crate::sol::svc::svce_calc) fn revise_on_item_remove(
        &self,
        added_item: &SolItem,
        sol_view: &SolView,
    ) -> bool {
        let src_item = sol_view.items.get_item(&self.affector_item_id).unwrap();
        self.val_getter.revise_on_item_remove(src_item, added_item)
    }
}
