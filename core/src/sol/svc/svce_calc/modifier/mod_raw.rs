use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, SolItemId},
    ec,
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
pub(in crate::sol::svc::svce_calc) struct SolRawModifier {
    pub(in crate::sol::svc::svce_calc) kind: SolModifierKind,
    pub(in crate::sol::svc::svce_calc) affector_item_id: SolItemId,
    pub(in crate::sol::svc::svce_calc) effect_id: EEffectId,
    affector_value: SolAffectorValue,
    pub(in crate::sol::svc::svce_calc) op: SolOp,
    pub(in crate::sol::svc::svce_calc) aggr_mode: SolAggrMode,
    pub(in crate::sol::svc::svce_calc) affectee_filter: SolAffecteeFilter,
    pub(in crate::sol::svc::svce_calc) affectee_attr_id: EAttrId,
    // Buff-related
    pub(in crate::sol::svc::svce_calc) buff_type_attr_id: Option<EAttrId>,
    // Projection-related
    pub(in crate::sol::svc::svce_calc) resist_attr_id: Option<EAttrId>,
    pub(in crate::sol::svc::svce_calc) optimal_attr_id: Option<EAttrId>,
    pub(in crate::sol::svc::svce_calc) falloff_attr_id: Option<EAttrId>,
}
impl SolRawModifier {
    pub(super) fn new(
        kind: SolModifierKind,
        affector_item_id: SolItemId,
        effect_id: EEffectId,
        affector_value: SolAffectorValue,
        op: SolOp,
        aggr_mode: SolAggrMode,
        affectee_filter: SolAffecteeFilter,
        affectee_attr_id: EAttrId,
        buff_type_attr_id: Option<EAttrId>,
        resist_attr_id: Option<EAttrId>,
        optimal_attr_id: Option<EAttrId>,
        falloff_attr_id: Option<EAttrId>,
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
            buff_type_attr_id,
            resist_attr_id,
            optimal_attr_id,
            falloff_attr_id,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn from_a_modifier(
        affector_item: &SolItem,
        a_effect: &ad::AEffect,
        a_modifier: &ad::AEffectModifier,
    ) -> Option<Self> {
        let affectee_filter =
            SolAffecteeFilter::from_a_effect_affectee_filter(&a_modifier.affectee_filter, affector_item);
        let kind = match get_mod_kind(a_effect, &affectee_filter) {
            Some(kind) => kind,
            None => return None,
        };
        let (resist_attr_id, optimal_attr_id, falloff_attr_id) = match kind.is_projectable() {
            true => (
                get_resist_attr_id(affector_item, a_effect),
                a_effect.range_attr_id,
                a_effect.falloff_attr_id,
            ),
            false => (None, None, None),
        };
        Some(Self::new(
            kind,
            affector_item.get_id(),
            a_effect.id,
            SolAffectorValue::AttrId(a_modifier.affector_attr_id),
            (&a_modifier.op).into(),
            SolAggrMode::Stack,
            affectee_filter,
            a_modifier.affectee_attr_id,
            None,
            resist_attr_id,
            optimal_attr_id,
            falloff_attr_id,
        ))
    }
    pub(in crate::sol::svc::svce_calc) fn from_a_buff(
        affector_item: &SolItem,
        a_effect: &ad::AEffect,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffModifier,
        affector_attr_id: EAttrId,
        domain: SolDomain,
        buff_type_attr_id: Option<EAttrId>,
    ) -> Option<Self> {
        let affectee_filter =
            SolAffecteeFilter::from_a_buff_affectee_filter(&a_mod.affectee_filter, domain, affector_item);
        let kind = match get_mod_kind(a_effect, &affectee_filter) {
            Some(kind) => kind,
            None => return None,
        };
        let (resist_attr_id, optimal_attr_id, falloff_attr_id) = match kind.is_projectable() {
            true => (
                get_resist_attr_id(affector_item, a_effect),
                a_effect.range_attr_id,
                a_effect.falloff_attr_id,
            ),
            false => (None, None, None),
        };
        Some(Self::new(
            kind,
            affector_item.get_id(),
            a_effect.id,
            SolAffectorValue::AttrId(affector_attr_id),
            (&a_buff.op).into(),
            SolAggrMode::from_a_buff(a_buff),
            affectee_filter,
            a_mod.affectee_attr_id,
            buff_type_attr_id,
            resist_attr_id,
            optimal_attr_id,
            falloff_attr_id,
        ))
    }
    pub(in crate::sol::svc::svce_calc) fn get_affector_attr_id(&self) -> Option<EAttrId> {
        self.affector_value.get_affector_attr_id()
    }
    pub(in crate::sol::svc::svce_calc) fn get_affectors(&self, sol_view: &SolView) -> Vec<(SolItemId, EAttrId)> {
        self.affector_value.get_affectors(sol_view, &self.affector_item_id)
    }
    pub(in crate::sol::svc::svce_calc) fn get_mod_val(&self, svc: &mut SolSvcs, sol_view: &SolView) -> Result<AttrVal> {
        self.affector_value
            .get_mod_val(svc, sol_view, &self.affector_item_id, &self.effect_id)
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

fn get_mod_kind(effect: &ad::AEffect, affectee_filter: &SolAffecteeFilter) -> Option<SolModifierKind> {
    if let SolAffecteeFilter::Direct(domain) = affectee_filter {
        if matches!(domain, SolDomain::Item | SolDomain::Other) {
            return Some(SolModifierKind::Local);
        }
    }
    match (effect.category, &effect.buff) {
        // Local modifications
        (ec::effcats::PASSIVE | ec::effcats::ACTIVE | ec::effcats::ONLINE | ec::effcats::OVERLOAD, None) => {
            Some(SolModifierKind::Local)
        }
        // Buffs
        (ec::effcats::ACTIVE, Some(buff_info)) => match buff_info.scope {
            ad::AEffectBuffScope::FleetShips => Some(SolModifierKind::FleetBuff),
            _ => Some(SolModifierKind::Buff),
        },
        // Lib system-wide effects are EVE system effects and buffs
        (ec::effcats::SYSTEM, None) => Some(SolModifierKind::System),
        // Targeted effects
        (ec::effcats::TARGET, None) => Some(SolModifierKind::Targeted),
        _ => None,
    }
}

fn get_resist_attr_id(item: &SolItem, effect: &ad::AEffect) -> Option<EAttrId> {
    match effect.resist_attr_id {
        Some(resist_attr_id) => Some(resist_attr_id),
        None => match item.get_orig_attrs() {
            Ok(attrs) => match attrs.get(&ec::attrs::REMOTE_RESISTANCE_ID).map(|v| *v as EAttrId) {
                Some(attr_id) if attr_id != 0 => Some(attr_id),
                _ => None,
            },
            _ => None,
        },
    }
}
