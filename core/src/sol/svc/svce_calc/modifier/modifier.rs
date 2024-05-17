use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, SolItemId},
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{SolAffecteeFilter, SolAggrMode, SolDomain, SolModifierKind, SolOp},
            SolSvcs,
        },
        SolView,
    },
    util::Result,
};

use super::SolAffectorValue;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) struct SolModifier {
    pub(in crate::sol::svc::svce_calc) kind: SolModifierKind,
    pub(in crate::sol::svc::svce_calc) affector_item_id: SolItemId,
    pub(in crate::sol::svc::svce_calc) effect_id: EEffectId,
    affector_value: SolAffectorValue,
    pub(in crate::sol::svc::svce_calc) op: SolOp,
    pub(in crate::sol::svc::svce_calc) aggr_mode: SolAggrMode,
    pub(in crate::sol::svc::svce_calc) affectee_filter: SolAffecteeFilter,
    pub(in crate::sol::svc::svce_calc) affectee_attr_id: EAttrId,
}
impl SolModifier {
    pub(super) fn new(
        kind: SolModifierKind,
        affector_item_id: SolItemId,
        effect_id: EEffectId,
        affector_value: SolAffectorValue,
        op: SolOp,
        aggr_mode: SolAggrMode,
        affectee_filter: SolAffecteeFilter,
        affectee_attr_id: EAttrId,
    ) -> Self {
        Self {
            kind,
            affector_item_id,
            effect_id,
            affector_value,
            op,
            aggr_mode,
            affectee_filter,
            affectee_attr_id,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn from_a_effect(
        affector_item: &SolItem,
        a_effect: &ad::AEffect,
        a_modifier: &ad::AEffectModifier,
        kind: SolModifierKind,
    ) -> Self {
        Self::new(
            kind,
            affector_item.get_id(),
            a_effect.id,
            SolAffectorValue::AttrId(a_modifier.affector_attr_id),
            (&a_modifier.op).into(),
            SolAggrMode::Stack,
            SolAffecteeFilter::from_a_effect_tgt_filter(&a_modifier.affectee_filter, affector_item),
            a_modifier.affectee_attr_id,
        )
    }
    pub(in crate::sol::svc::svce_calc) fn from_a_buff(
        affector_item: &SolItem,
        a_effect: &ad::AEffect,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffModifier,
        affector_attr_id: EAttrId,
        mod_kind: SolModifierKind,
        domain: SolDomain,
    ) -> Self {
        Self::new(
            mod_kind,
            affector_item.get_id(),
            a_effect.id,
            SolAffectorValue::AttrId(affector_attr_id),
            (&a_buff.op).into(),
            SolAggrMode::from_a_buff(a_buff),
            SolAffecteeFilter::from_a_buff_tgt_filter(&a_mod.affectee_filter, domain, affector_item),
            a_mod.affectee_attr_id,
        )
    }
    pub(in crate::sol::svc::svce_calc) fn get_affector_attr_id(&self) -> Option<EAttrId> {
        self.affector_value.get_affector_attr_id()
    }
    pub(in crate::sol::svc::svce_calc) fn get_affectors(&self, sol_view: &SolView) -> Vec<(SolItemId, EAttrId)> {
        self.affector_value.get_affectors(sol_view, &self.affector_item_id)
    }
    pub(in crate::sol::svc::svce_calc) fn get_mod_val(
        &self,
        svc: &mut SolSvcs,
        sol_view: &SolView,
        range: Option<AttrVal>,
    ) -> Result<AttrVal> {
        self.affector_value.get_mod_val(svc, sol_view, &self.affector_item_id)
    }
    pub(in crate::sol::svc::svce_calc) fn on_effect_stop(&self, svc: &mut SolSvcs, sol_view: &SolView) {
        self.affector_value
            .on_effect_stop(svc, sol_view, &self.affector_item_id)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::sol::svc::svce_calc) fn needs_revision_on_item_add(&self) -> bool {
        self.affector_value.revisable_on_item_add()
    }
    pub(in crate::sol::svc::svce_calc) fn needs_revision_on_item_remove(&self) -> bool {
        self.affector_value.revisable_on_item_remove()
    }
    pub(in crate::sol::svc::svce_calc) fn revise_on_item_add(&self, added_item: &SolItem, sol_view: &SolView) -> bool {
        let affector_item = sol_view.items.get_item(&self.affector_item_id).unwrap();
        self.affector_value.revise_on_item_add(affector_item, added_item)
    }
    pub(in crate::sol::svc::svce_calc) fn revise_on_item_remove(
        &self,
        added_item: &SolItem,
        sol_view: &SolView,
    ) -> bool {
        let affector_item = sol_view.items.get_item(&self.affector_item_id).unwrap();
        self.affector_value.revise_on_item_remove(affector_item, added_item)
    }
}
