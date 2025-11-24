use std::hash::{Hash, Hasher};

use smallvec::SmallVec;

use super::AffectorValue;
use crate::{
    ac, ad,
    def::AttrVal,
    misc::EffectSpec,
    nd::NProjMultGetter,
    rd,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AffectorInfo, AggrMode, Calc, ItemAddReviser, ItemRemoveReviser, Location, ModifierKind, Op,
        },
        eff_funcs,
    },
    ud::{UItem, UItemKey},
};

#[derive(Copy, Clone)]
pub(crate) struct RawModifier {
    pub(crate) kind: ModifierKind,
    pub(crate) affector_espec: EffectSpec,
    pub(crate) affector_value: AffectorValue,
    pub(crate) op: Op,
    pub(crate) aggr_mode: AggrMode,
    pub(crate) affectee_filter: AffecteeFilter,
    pub(crate) affectee_attr_id: ad::AAttrId,
    // Buff-related
    pub(crate) buff_type_attr_id: Option<ad::AAttrId> = None,
    // Projection-related
    pub(crate) proj_mult_getter: Option<NProjMultGetter> = None,
    pub(crate) proj_attr_ids: [Option<ad::AAttrId>; 2] = [None, None],
    pub(crate) resist_attr_id: Option<ad::AAttrId> = None,
}
impl PartialEq for RawModifier {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
            && self.affector_espec.eq(&other.affector_espec)
            && self.affector_value.eq(&other.affector_value)
            && self.op.eq(&other.op)
            && self.aggr_mode.eq(&other.aggr_mode)
            && self.affectee_filter.eq(&other.affectee_filter)
            && self.affectee_attr_id.eq(&other.affectee_attr_id)
            && self.buff_type_attr_id.eq(&other.buff_type_attr_id)
    }
}
impl Eq for RawModifier {}
impl Hash for RawModifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.affector_espec.hash(state);
        self.affector_value.hash(state);
        self.op.hash(state);
        self.aggr_mode.hash(state);
        self.affectee_filter.hash(state);
        self.affectee_attr_id.hash(state);
        self.buff_type_attr_id.hash(state);
    }
}
impl RawModifier {
    pub(in crate::svc::calc) fn try_from_effect_mod(
        affector_key: UItemKey,
        affector_item: &UItem,
        effect: &rd::REffect,
        effect_mod: &ad::AEffectModifier,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_effect_affectee_filter(&effect_mod.affectee_filter, affector_item);
        let kind = get_mod_kind(effect, &affectee_filter)?;
        // Targeted effects are affected by resists
        let resist_attr_id = match kind {
            ModifierKind::Targeted => eff_funcs::get_resist_attr_id(affector_item, effect),
            _ => None,
        };
        Some(Self {
            kind,
            affector_espec: EffectSpec::new(affector_key, effect.get_key()),
            affector_value: AffectorValue::AttrId(effect_mod.affector_attr_id),
            op: (&effect_mod.op).into(),
            aggr_mode: AggrMode::Stack,
            affectee_filter,
            affectee_attr_id: effect_mod.affectee_attr_id,
            buff_type_attr_id: None,
            proj_mult_getter: effect.get_modifier_proj_mult_getter(),
            proj_attr_ids: effect.get_modifier_proj_attr_ids(),
            resist_attr_id,
            ..
        })
    }
    pub(in crate::svc::calc) fn try_from_buff_regular(
        affector_key: UItemKey,
        affector_item: &UItem,
        effect: &rd::REffect,
        buff: &rd::RBuff,
        buff_mod: &ad::ABuffModifier,
        affector_attr_id: ad::AAttrId,
        loc: Location,
        buff_type_attr_id: Option<ad::AAttrId>,
    ) -> Option<Self> {
        RawModifier::from_buff(
            affector_key,
            affector_item,
            effect,
            buff,
            buff_mod,
            AffectorValue::AttrId(affector_attr_id),
            loc,
            buff_type_attr_id,
        )
    }
    pub(in crate::svc::calc) fn try_from_buff_hardcoded(
        affector_key: UItemKey,
        affector_item: &UItem,
        effect: &rd::REffect,
        buff: &rd::RBuff,
        buff_mod: &ad::ABuffModifier,
        affector_mod_val: AttrVal,
        loc: Location,
    ) -> Option<Self> {
        RawModifier::from_buff(
            affector_key,
            affector_item,
            effect,
            buff,
            buff_mod,
            AffectorValue::Hardcoded(affector_mod_val),
            loc,
            None,
        )
    }
    fn from_buff(
        affector_key: UItemKey,
        affector_item: &UItem,
        effect: &rd::REffect,
        buff: &rd::RBuff,
        buff_mod: &ad::ABuffModifier,
        affector_value: AffectorValue,
        loc: Location,
        buff_type_attr_id: Option<ad::AAttrId>,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_buff_affectee_filter(&buff_mod.affectee_filter, loc, affector_item);
        let kind = get_mod_kind(effect, &affectee_filter)?;
        let resist_attr_id = match kind {
            ModifierKind::Buff => eff_funcs::get_resist_attr_id(affector_item, effect),
            _ => None,
        };
        Some(Self {
            kind,
            affector_espec: EffectSpec::new(affector_key, effect.get_key()),
            affector_value,
            op: (&buff.get_op()).into(),
            aggr_mode: AggrMode::from_r_buff(buff),
            affectee_filter,
            affectee_attr_id: buff_mod.affectee_attr_id,
            buff_type_attr_id,
            proj_mult_getter: effect.get_modifier_proj_mult_getter(),
            proj_attr_ids: effect.get_modifier_proj_attr_ids(),
            resist_attr_id,
            ..
        })
    }
    pub(in crate::svc::calc) fn get_affector_attr_id(&self) -> Option<ad::AAttrId> {
        self.affector_value.get_affector_a_attr_id()
    }
    pub(in crate::svc::calc) fn get_affector_info(&self, ctx: SvcCtx) -> SmallVec<AffectorInfo, 1> {
        self.affector_value.get_affector_info(ctx, self.affector_espec.item_key)
    }
    pub(in crate::svc::calc) fn get_mod_val(&self, calc: &mut Calc, ctx: SvcCtx) -> Option<AttrVal> {
        self.affector_value.get_mod_val(calc, ctx, self.affector_espec)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::svc::calc) fn get_item_add_reviser(&self) -> Option<ItemAddReviser> {
        self.affector_value.get_item_add_reviser()
    }
    pub(in crate::svc::calc) fn get_item_remove_reviser(&self) -> Option<ItemRemoveReviser> {
        self.affector_value.get_item_remove_reviser()
    }
}

fn get_mod_kind(r_effect: &rd::REffect, affectee_filter: &AffecteeFilter) -> Option<ModifierKind> {
    if let AffecteeFilter::Direct(loc) = affectee_filter
        && matches!(loc, Location::Item | Location::Other)
    {
        return Some(ModifierKind::Local);
    }
    match (r_effect.get_category(), &r_effect.get_buff_info()) {
        // Local modifications
        (ac::effcats::PASSIVE | ac::effcats::ACTIVE | ac::effcats::ONLINE | ac::effcats::OVERLOAD, None) => {
            Some(ModifierKind::Local)
        }
        // Buffs
        (ac::effcats::ACTIVE, Some(a_buff_info)) => match a_buff_info.scope.fleet_only {
            true => Some(ModifierKind::FleetBuff),
            false => Some(ModifierKind::Buff),
        },
        // Lib system-wide effects are EVE system effects and buffs
        (ac::effcats::SYSTEM, None) => Some(ModifierKind::System),
        // Targeted effects
        (ac::effcats::TARGET, None) => Some(ModifierKind::Targeted),
        _ => None,
    }
}
