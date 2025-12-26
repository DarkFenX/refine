use std::hash::{Hash, Hasher};

use smallvec::SmallVec;

use super::AffectorValue;
use crate::{
    ac,
    ad::AEffectCatId,
    def::AttrVal,
    misc::EffectSpec,
    nd::NProjMultGetter,
    rd::{RAttrKey, RBuff, RBuffModifier, REffect, REffectBuffScope, REffectModifier},
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Affector, AggrMode, Calc, CalcOp, ItemAddReviser, ItemRemoveReviser, Location, ModifierKind,
        },
        funcs,
    },
    ud::{UItem, UItemKey},
};

#[derive(Copy, Clone)]
pub(crate) struct RawModifier {
    pub(crate) kind: ModifierKind,
    pub(crate) affector_espec: EffectSpec,
    pub(crate) affector_value: AffectorValue,
    pub(crate) op: CalcOp,
    pub(crate) aggr_mode: AggrMode,
    pub(crate) affectee_filter: AffecteeFilter,
    pub(crate) affectee_attr_key: RAttrKey,
    // Buff-related
    pub(crate) buff_type_attr_key: Option<RAttrKey> = None,
    // Projection-related
    pub(crate) proj_mult_getter: Option<NProjMultGetter> = None,
    pub(crate) proj_attr_keys: [Option<RAttrKey>; 2] = [None, None],
    pub(crate) resist_attr_key: Option<RAttrKey> = None,
}
impl PartialEq for RawModifier {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
            && self.affector_espec.eq(&other.affector_espec)
            && self.affector_value.eq(&other.affector_value)
            && self.op.eq(&other.op)
            && self.aggr_mode.eq(&other.aggr_mode)
            && self.affectee_filter.eq(&other.affectee_filter)
            && self.affectee_attr_key.eq(&other.affectee_attr_key)
            && self.buff_type_attr_key.eq(&other.buff_type_attr_key)
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
        self.affectee_attr_key.hash(state);
        self.buff_type_attr_key.hash(state);
    }
}
impl RawModifier {
    pub(in crate::svc::calc) fn try_from_effect_mod(
        affector_key: UItemKey,
        affector_item: &UItem,
        effect: &REffect,
        effect_mod: &REffectModifier,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_effect_affectee_filter(&effect_mod.affectee_filter, affector_item);
        let kind = get_effect_mod_kind(effect.category, &affectee_filter)?;
        // Only targeted effects can be affected by resists
        let resist_attr_key = match kind {
            ModifierKind::Targeted => funcs::get_resist_attr_key(affector_item, effect),
            _ => None,
        };
        Some(Self {
            kind,
            affector_espec: EffectSpec::new(affector_key, effect.key),
            affector_value: AffectorValue::Attr(effect_mod.affector_attr_key),
            op: (&effect_mod.op).into(),
            aggr_mode: AggrMode::Stack,
            affectee_filter,
            affectee_attr_key: effect_mod.affectee_attr_key,
            buff_type_attr_key: None,
            proj_mult_getter: effect.modifier_proj_mult_getter,
            proj_attr_keys: effect.modifier_proj_attr_keys,
            resist_attr_key,
            ..
        })
    }
    pub(in crate::svc::calc) fn try_from_buff_with_attr(
        affector_key: UItemKey,
        affector_item: &UItem,
        effect: &REffect,
        buff: &RBuff,
        buff_scope: &REffectBuffScope,
        buff_mod: &RBuffModifier,
        buff_type_attr_key: Option<RAttrKey>,
        buff_str_attr_key: RAttrKey,
    ) -> Option<Self> {
        RawModifier::try_from_buff(
            affector_key,
            affector_item,
            effect,
            buff,
            buff_scope,
            buff_mod,
            buff_type_attr_key,
            AffectorValue::Attr(buff_str_attr_key),
        )
    }
    pub(in crate::svc::calc) fn try_from_buff_with_hardcoded(
        affector_key: UItemKey,
        affector_item: &UItem,
        effect: &REffect,
        buff: &RBuff,
        buff_scope: &REffectBuffScope,
        buff_mod: &RBuffModifier,
        buff_str: AttrVal,
    ) -> Option<Self> {
        RawModifier::try_from_buff(
            affector_key,
            affector_item,
            effect,
            buff,
            buff_scope,
            buff_mod,
            None,
            AffectorValue::Hardcoded(buff_str),
        )
    }
    fn try_from_buff(
        affector_key: UItemKey,
        affector_item: &UItem,
        effect: &REffect,
        buff: &RBuff,
        buff_scope: &REffectBuffScope,
        buff_mod: &RBuffModifier,
        buff_type_attr_key: Option<RAttrKey>,
        buff_str: AffectorValue,
    ) -> Option<Self> {
        if effect.category != ac::effcats::ACTIVE {
            return None;
        }
        Some(match buff_scope {
            // Special processing for carrier scope. It is unknown how those self-buffs work on
            // non-ship items, since EVE does not have those in game, but we convert those into
            // local modifiers which affect just ship for simplicity of further processing
            REffectBuffScope::Carrier => Self {
                kind: ModifierKind::Local,
                affector_espec: EffectSpec::new(affector_key, effect.key),
                affector_value: buff_str,
                op: (&buff.op).into(),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::Ship,
                    affector_item,
                ),
                affectee_attr_key: buff_mod.affectee_attr_key,
                buff_type_attr_key,
                ..
            },
            // Projected modifiers can be range-reduced and resisted
            REffectBuffScope::Projected(item_list_key) => Self {
                kind: ModifierKind::Buff,
                affector_espec: EffectSpec::new(affector_key, effect.key),
                affector_value: buff_str,
                op: (&buff.op).into(),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::ItemList(*item_list_key),
                    affector_item,
                ),
                affectee_attr_key: buff_mod.affectee_attr_key,
                buff_type_attr_key,
                proj_mult_getter: effect.modifier_proj_mult_getter,
                proj_attr_keys: effect.modifier_proj_attr_keys,
                resist_attr_key: funcs::get_resist_attr_key(affector_item, effect),
                ..
            },
            // Fleet buffs cannot be resisted and range-reduced regardless of what effect says
            REffectBuffScope::Fleet(item_list_key) => Self {
                kind: ModifierKind::FleetBuff,
                affector_espec: EffectSpec::new(affector_key, effect.key),
                affector_value: buff_str,
                op: (&buff.op).into(),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::ItemList(*item_list_key),
                    affector_item,
                ),
                affectee_attr_key: buff_mod.affectee_attr_key,
                buff_type_attr_key,
                ..
            },
        })
    }
    pub(in crate::svc::calc) fn get_affector_attr_key(&self) -> Option<RAttrKey> {
        self.affector_value.get_affector_attr_key()
    }
    pub(in crate::svc::calc) fn get_affector_info(&self, ctx: SvcCtx) -> SmallVec<Affector, 1> {
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

fn get_effect_mod_kind(effect_cat: AEffectCatId, affectee_filter: &AffecteeFilter) -> Option<ModifierKind> {
    if let AffecteeFilter::Direct(loc) = affectee_filter
        && let Location::Item | Location::Other = loc
    {
        return Some(ModifierKind::Local);
    }
    match effect_cat {
        ac::effcats::PASSIVE | ac::effcats::ONLINE | ac::effcats::ACTIVE | ac::effcats::OVERLOAD => {
            Some(ModifierKind::Local)
        }
        ac::effcats::SYSTEM => Some(ModifierKind::System),
        ac::effcats::TARGET => Some(ModifierKind::Targeted),
        _ => None,
    }
}
