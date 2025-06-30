use smallvec::SmallVec;

use super::AffectorValue;
use crate::{
    ac, ad,
    def::{AttrVal, ItemKey},
    misc::EffectSpec,
    svc::{
        SvcCtx,
        calc::{AffecteeFilter, AffectorInfo, AggrMode, Calc, Location, ModifierKind, Op},
        get_resist_a_attr_id,
    },
    uad::UadItem,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct RawModifier {
    pub(crate) kind: ModifierKind,
    pub(crate) affector_espec: EffectSpec,
    pub(crate) affector_value: AffectorValue,
    pub(crate) op: Op,
    pub(crate) aggr_mode: AggrMode,
    pub(crate) affectee_filter: AffecteeFilter,
    pub(crate) affectee_a_attr_id: ad::AAttrId,
    // Buff-related
    pub(crate) buff_type_a_attr_id: Option<ad::AAttrId> = None,
    // Projection-related
    pub(crate) resist_a_attr_id: Option<ad::AAttrId> = None,
    pub(crate) optimal_a_attr_id: Option<ad::AAttrId> = None,
    pub(crate) falloff_a_attr_id: Option<ad::AAttrId> = None,
}
impl RawModifier {
    pub(in crate::svc::calc) fn try_from_a_modifier(
        affector_item_key: ItemKey,
        affector_item: &UadItem,
        a_effect: &ad::AEffectRt,
        a_modifier: &ad::AEffectModifier,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_a_effect_affectee_filter(&a_modifier.affectee_filter, affector_item);
        let kind = get_mod_kind(a_effect, &affectee_filter)?;
        // Targeted effects are affected by both range and resists
        let (resist_a_attr_id, optimal_a_attr_id, falloff_a_attr_id) = match kind {
            ModifierKind::Targeted => (
                get_resist_a_attr_id(affector_item, a_effect),
                a_effect.ae.range_attr_id,
                a_effect.ae.falloff_attr_id,
            ),
            _ => (None, None, None),
        };
        Some(Self {
            kind,
            affector_espec: EffectSpec::new(affector_item_key, a_effect.ae.id),
            affector_value: AffectorValue::AttrId(a_modifier.affector_attr_id),
            op: (&a_modifier.op).into(),
            aggr_mode: AggrMode::Stack,
            affectee_filter,
            affectee_a_attr_id: a_modifier.affectee_attr_id,
            buff_type_a_attr_id: None,
            resist_a_attr_id,
            optimal_a_attr_id,
            falloff_a_attr_id,
        })
    }
    pub(in crate::svc::calc) fn try_from_a_buff_regular(
        affector_item_key: ItemKey,
        affector_item: &UadItem,
        a_effect: &ad::AEffectRt,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffModifier,
        affector_a_attr_id: ad::AAttrId,
        loc: Location,
        buff_type_a_attr_id: Option<ad::AAttrId>,
    ) -> Option<Self> {
        RawModifier::from_a_buff(
            affector_item_key,
            affector_item,
            a_effect,
            a_buff,
            a_mod,
            AffectorValue::AttrId(affector_a_attr_id),
            loc,
            buff_type_a_attr_id,
        )
    }
    pub(in crate::svc::calc) fn try_from_a_buff_hardcoded(
        affector_item_key: ItemKey,
        affector_item: &UadItem,
        a_effect: &ad::AEffectRt,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffModifier,
        affector_mod_val: AttrVal,
        loc: Location,
    ) -> Option<Self> {
        RawModifier::from_a_buff(
            affector_item_key,
            affector_item,
            a_effect,
            a_buff,
            a_mod,
            AffectorValue::Hardcoded(affector_mod_val),
            loc,
            None,
        )
    }
    fn from_a_buff(
        affector_item_key: ItemKey,
        affector_item: &UadItem,
        a_effect: &ad::AEffectRt,
        a_buff: &ad::ABuff,
        a_mod: &ad::ABuffModifier,
        affector_value: AffectorValue,
        loc: Location,
        buff_type_a_attr_id: Option<ad::AAttrId>,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_a_buff_affectee_filter(&a_mod.affectee_filter, loc, affector_item);
        let kind = get_mod_kind(a_effect, &affectee_filter)?;
        let (resist_a_attr_id, optimal_a_attr_id) = match kind {
            ModifierKind::Buff => (get_resist_a_attr_id(affector_item, a_effect), a_effect.ae.range_attr_id),
            _ => (None, None),
        };
        Some(Self {
            kind,
            affector_espec: EffectSpec::new(affector_item_key, a_effect.ae.id),
            affector_value,
            op: (&a_buff.op).into(),
            aggr_mode: AggrMode::from_a_buff(a_buff),
            affectee_filter,
            affectee_a_attr_id: a_mod.affectee_attr_id,
            buff_type_a_attr_id,
            resist_a_attr_id,
            optimal_a_attr_id,
            // Modifiers created from buffs never define falloff - buffs either apply fully, or they
            // don't
            falloff_a_attr_id: None,
        })
    }
    pub(in crate::svc::calc) fn get_affector_a_attr_id(&self) -> Option<ad::AAttrId> {
        self.affector_value.get_affector_a_attr_id()
    }
    pub(in crate::svc::calc) fn get_affector_info(&self, ctx: &SvcCtx) -> SmallVec<AffectorInfo, 1> {
        self.affector_value.get_affector_info(ctx, self.affector_espec.item_key)
    }
    pub(in crate::svc::calc) fn get_mod_val(&self, calc: &mut Calc, ctx: &SvcCtx) -> Option<AttrVal> {
        self.affector_value.get_mod_val(calc, ctx, self.affector_espec)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::svc::calc) fn needs_revision_on_item_add(&self) -> bool {
        self.affector_value.revisable_on_item_add()
    }
    pub(in crate::svc::calc) fn needs_revision_on_item_remove(&self) -> bool {
        self.affector_value.revisable_on_item_remove()
    }
    pub(in crate::svc::calc) fn revise_on_item_add(
        &self,
        ctx: &SvcCtx,
        added_item_key: ItemKey,
        added_item: &UadItem,
    ) -> bool {
        self.affector_value
            .revise_on_item_add(ctx, self.affector_espec.item_key, added_item_key, added_item)
    }
    pub(in crate::svc::calc) fn revise_on_item_remove(
        &self,
        ctx: &SvcCtx,
        removed_item_key: ItemKey,
        removed_item: &UadItem,
    ) -> bool {
        self.affector_value
            .revise_on_item_remove(ctx, self.affector_espec.item_key, removed_item_key, removed_item)
    }
}

fn get_mod_kind(a_effect: &ad::AEffectRt, a_affectee_filter: &AffecteeFilter) -> Option<ModifierKind> {
    if let AffecteeFilter::Direct(loc) = a_affectee_filter
        && matches!(loc, Location::Item | Location::Other)
    {
        return Some(ModifierKind::Local);
    }
    match (a_effect.ae.category, &a_effect.ae.buff) {
        // Local modifications
        (ac::effcats::PASSIVE | ac::effcats::ACTIVE | ac::effcats::ONLINE | ac::effcats::OVERLOAD, None) => {
            Some(ModifierKind::Local)
        }
        // Buffs
        (ac::effcats::ACTIVE, Some(a_buff_info)) => match a_buff_info.scope {
            ad::AEffectBuffScope::FleetShips => Some(ModifierKind::FleetBuff),
            _ => Some(ModifierKind::Buff),
        },
        // Lib system-wide effects are EVE system effects and buffs
        (ac::effcats::SYSTEM, None) => Some(ModifierKind::System),
        // Targeted effects
        (ac::effcats::TARGET, None) => Some(ModifierKind::Targeted),
        _ => None,
    }
}
