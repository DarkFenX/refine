use std::hash::{Hash, Hasher};

use smallvec::SmallVec;

use crate::{
    ad::AEffectCatId,
    dbg::DebugResult,
    misc::EffectSpec,
    num::Value,
    rd::{
        RAttrId, RBuff, RBuffModifier, REffect, REffectBuffScope, REffectModStrength, REffectModifier,
        REffectProjModSpec,
    },
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AggrMode, Calc, CalcModInfoAffector, CalcOp, ItemAddRemoveReviser, Location, ModifierKind,
            modifier::ModStrength,
        },
    },
    ud::{UData, UItem, UItemId},
};

#[derive(Copy, Clone)]
pub(in crate::svc::calc) struct RawModifier {
    pub(in crate::svc::calc) kind: ModifierKind,
    pub(in crate::svc::calc) affector_espec: EffectSpec,
    pub(in crate::svc::calc::modifier) strength: ModStrength,
    pub(in crate::svc::calc) op: CalcOp,
    pub(in crate::svc::calc) aggr_mode: AggrMode,
    pub(in crate::svc::calc) affectee_filter: AffecteeFilter,
    pub(in crate::svc::calc) affectee_attr_rid: RAttrId,
    // Buff-related
    pub(in crate::svc::calc) buff_type_attr_rid: Option<RAttrId> = None,
    // Projection-related
    pub(in crate::svc::calc) proj_spec: Option<REffectProjModSpec> = None,
    pub(in crate::svc::calc) resist_attr_rid: Option<RAttrId> = None,
}
impl PartialEq for RawModifier {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
            && self.affector_espec.eq(&other.affector_espec)
            && self.strength.eq(&other.strength)
            && self.op.eq(&other.op)
            && self.aggr_mode.eq(&other.aggr_mode)
            && self.affectee_filter.eq(&other.affectee_filter)
            && self.affectee_attr_rid.eq(&other.affectee_attr_rid)
            && self.buff_type_attr_rid.eq(&other.buff_type_attr_rid)
    }
}
impl Eq for RawModifier {}
impl Hash for RawModifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.affector_espec.hash(state);
        self.strength.hash(state);
        self.op.hash(state);
        self.aggr_mode.hash(state);
        self.affectee_filter.hash(state);
        self.affectee_attr_rid.hash(state);
        self.buff_type_attr_rid.hash(state);
    }
}
impl RawModifier {
    pub(in crate::svc::calc) fn try_from_effect_mod(
        u_data: &UData,
        affector_uid: UItemId,
        affector_item: &UItem,
        effect: &REffect,
        effect_mod: &REffectModifier,
    ) -> Option<Self> {
        let affectee_filter = AffecteeFilter::from_effect_affectee_filter(&effect_mod.affectee_filter, affector_item);
        let kind = get_effect_mod_kind(effect.category, &affectee_filter)?;
        // Only targeted effects can be affected by projected modifier spec (range/resist reduction)
        let (proj_spec, resist_attr_rid) = match kind {
            ModifierKind::Targeted => (
                effect.proj_mod,
                effect
                    .proj_mod
                    .and_then(|v| v.resist)
                    .and_then(|v| v.get_attr_rid(u_data, affector_uid)),
            ),
            _ => (None, None),
        };
        Some(Self {
            kind,
            affector_espec: EffectSpec::new(affector_uid, effect.rid),
            strength: match effect_mod.strength {
                REffectModStrength::Attr(attr_rid) => ModStrength::Attr(attr_rid),
                REffectModStrength::Hardcoded(value) => ModStrength::Hardcoded(value),
            },
            op: CalcOp::from_a_op(effect_mod.op),
            aggr_mode: AggrMode::Stack,
            affectee_filter,
            affectee_attr_rid: effect_mod.affectee_attr_rid,
            buff_type_attr_rid: None,
            proj_spec,
            resist_attr_rid,
            ..
        })
    }
    pub(in crate::svc::calc) fn try_from_buff_with_attr(
        u_data: &UData,
        affector_uid: UItemId,
        affector_item: &UItem,
        effect: &REffect,
        buff: &RBuff,
        buff_scope: &REffectBuffScope,
        buff_mod: &RBuffModifier,
        buff_type_attr_rid: Option<RAttrId>,
        buff_str_attr_rid: RAttrId,
    ) -> Option<Self> {
        RawModifier::try_from_buff(
            u_data,
            affector_uid,
            affector_item,
            effect,
            buff,
            buff_scope,
            buff_mod,
            buff_type_attr_rid,
            ModStrength::Attr(buff_str_attr_rid),
        )
    }
    pub(in crate::svc::calc) fn try_from_buff_with_hardcoded(
        u_data: &UData,
        affector_rid: UItemId,
        affector_item: &UItem,
        effect: &REffect,
        buff: &RBuff,
        buff_scope: &REffectBuffScope,
        buff_mod: &RBuffModifier,
        buff_str: Value,
    ) -> Option<Self> {
        RawModifier::try_from_buff(
            u_data,
            affector_rid,
            affector_item,
            effect,
            buff,
            buff_scope,
            buff_mod,
            None,
            ModStrength::Hardcoded(buff_str),
        )
    }
    fn try_from_buff(
        u_data: &UData,
        affector_uid: UItemId,
        affector_item: &UItem,
        effect: &REffect,
        buff: &RBuff,
        buff_scope: &REffectBuffScope,
        buff_mod: &RBuffModifier,
        buff_type_attr_rid: Option<RAttrId>,
        buff_str: ModStrength,
    ) -> Option<Self> {
        if effect.category != AEffectCatId::ACTIVE {
            return None;
        }
        Some(match buff_scope {
            // Special processing for carrier scope. It is unknown how those self-buffs work on
            // non-ship items, since EVE does not have those in game, but we convert those into
            // local modifiers which affect just ship for simplicity of further processing
            REffectBuffScope::Carrier => Self {
                kind: ModifierKind::Local,
                affector_espec: EffectSpec::new(affector_uid, effect.rid),
                strength: buff_str,
                op: CalcOp::from_a_op(buff.op),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::Ship,
                    affector_item,
                ),
                affectee_attr_rid: buff_mod.affectee_attr_rid,
                buff_type_attr_rid,
                ..
            },
            // Projected modifiers can be range-reduced and resisted
            REffectBuffScope::Projected(item_list_rid) => Self {
                kind: ModifierKind::Buff,
                affector_espec: EffectSpec::new(affector_uid, effect.rid),
                strength: buff_str,
                op: CalcOp::from_a_op(buff.op),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::ItemList(*item_list_rid),
                    affector_item,
                ),
                affectee_attr_rid: buff_mod.affectee_attr_rid,
                buff_type_attr_rid,
                proj_spec: effect.proj_mod,
                resist_attr_rid: effect
                    .proj_mod
                    .and_then(|v| v.resist)
                    .and_then(|v| v.get_attr_rid(u_data, affector_uid)),
                ..
            },
            // Fleet buffs cannot be resisted and range-reduced regardless of what effect says
            REffectBuffScope::Fleet(item_list_rid) => Self {
                kind: ModifierKind::FleetBuff,
                affector_espec: EffectSpec::new(affector_uid, effect.rid),
                strength: buff_str,
                op: CalcOp::from_a_op(buff.op),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::ItemList(*item_list_rid),
                    affector_item,
                ),
                affectee_attr_rid: buff_mod.affectee_attr_rid,
                buff_type_attr_rid,
                ..
            },
        })
    }
    pub(in crate::svc::calc) fn get_affector_attr_rid(&self) -> Option<RAttrId> {
        self.strength.get_affector_attr_rid()
    }
    pub(in crate::svc::calc) fn get_affector_info(&self, ctx: SvcCtx) -> SmallVec<[CalcModInfoAffector; 1]> {
        self.strength.get_affector_info(ctx, self.affector_espec.item_uid)
    }
    pub(in crate::svc::calc) fn get_mod_val(&self, calc: &mut Calc, ctx: SvcCtx) -> Option<Value> {
        self.strength.get_strength(calc, ctx, self.affector_espec)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::svc::calc) fn get_item_add_remove_reviser(&self) -> Option<ItemAddRemoveReviser> {
        self.strength.get_item_add_remove_reviser()
    }
}

fn get_effect_mod_kind(effect_cat: AEffectCatId, affectee_filter: &AffecteeFilter) -> Option<ModifierKind> {
    if let AffecteeFilter::Direct(loc) = affectee_filter
        && let Location::Item | Location::Other = loc
    {
        return Some(ModifierKind::Local);
    }
    match effect_cat {
        AEffectCatId::PASSIVE | AEffectCatId::ONLINE | AEffectCatId::ACTIVE | AEffectCatId::OVERLOAD => {
            Some(ModifierKind::Local)
        }
        AEffectCatId::SYSTEM => Some(ModifierKind::System),
        AEffectCatId::TARGET => Some(ModifierKind::Targeted),
        _ => None,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RawModifier {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.affector_espec.consistency_check(u_data, true)?;
        self.strength.consistency_check(u_data)?;
        self.affectee_attr_rid.consistency_check(u_data)?;
        if let Some(attr_rid) = self.buff_type_attr_rid.as_ref() {
            attr_rid.consistency_check(u_data)?;
        }
        if let Some(mspec) = self.proj_spec.as_ref() {
            mspec.consistency_check(u_data)?;
        }
        if let Some(attr_rid) = self.resist_attr_rid.as_ref() {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
