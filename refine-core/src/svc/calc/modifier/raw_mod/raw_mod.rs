use smallvec::SmallVec;

use super::affector::{RawModAffector, RawModItemDependent};
use crate::{
    Value,
    ad::AEffectCatId,
    misc::EffectSpec,
    rd::{
        RAttrId, RBuff, RBuffModifier, REffect, REffectBuffFull, REffectBuffScope, REffectModifier, REffectProjModSpec,
    },
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, AggrMode, Calc, CalcModInfoAffector, CalcOp, ItemAddRemoveReviser, Location, ModifierKind,
            modifier::ModStrength,
        },
    },
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Definition & primary access interface
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "dev", derive(PartialEq))]
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
impl RawModifier {
    pub(in crate::svc::calc) fn get_affector_attr_rid(&self) -> Option<RAttrId> {
        self.strength.get_affector_attr_rid()
    }
    pub(in crate::svc::calc) fn get_affector_info(&self, ctx: &SvcCtx) -> SmallVec<[CalcModInfoAffector; 1]> {
        self.strength.get_affector_info(ctx, self.affector_espec.item_uid)
    }
    pub(in crate::svc::calc) fn get_mod_val(&self, calc: &mut Calc, ctx: &SvcCtx) -> Option<Value> {
        self.strength.get_strength(calc, ctx, self.affector_espec)
    }
    // Revision methods - define if modification value can change upon some action
    pub(in crate::svc::calc) fn get_item_add_remove_reviser(&self) -> Option<ItemAddRemoveReviser> {
        self.strength.get_item_add_remove_reviser()
    }
    // Dependency methods - define if modifier registers ad-hoc attribute dependencies with its
    // effect as source
    pub(in crate::svc::calc) fn adds_deps_with_source(&self) -> bool {
        self.strength.adds_deps_with_source()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RawModifier {
    pub(in crate::svc::calc) fn try_from_effect_mod(
        affector: RawModAffector,
        effect: &REffect,
        effect_mod: &REffectModifier,
    ) -> Result<Option<Self>, RawModItemDependent> {
        let affectee_filter = AffecteeFilter::try_from_effect_affectee_filter(&effect_mod.affectee_filter, affector)?;
        let Some(kind) = get_effect_mod_kind(effect.category, &affectee_filter) else {
            return Ok(None);
        };
        // Only targeted effects can be affected by projected modifier spec (range/resist reduction)
        let (proj_spec, resist_attr_rid) = match kind {
            ModifierKind::Targeted => (effect.proj_mod, get_resist_attr_rid(affector, effect)?),
            _ => (None, None),
        };
        Ok(Some(Self {
            kind,
            affector_espec: affector.get_espec(effect),
            strength: ModStrength::from_effect_mod_strength(effect_mod.strength),
            op: CalcOp::from_a_op(effect_mod.op),
            aggr_mode: AggrMode::Stack,
            affectee_filter,
            affectee_attr_rid: effect_mod.affectee_attr_rid,
            buff_type_attr_rid: None,
            proj_spec,
            resist_attr_rid,
            ..
        }))
    }
    pub(in crate::svc::calc) fn try_from_buff_full_mod(
        affector: RawModAffector,
        effect: &REffect,
        buff: &RBuff,
        buff_full: &REffectBuffFull,
        buff_mod: &RBuffModifier,
    ) -> Result<Option<Self>, RawModItemDependent> {
        Self::try_from_buff(
            affector,
            effect,
            buff,
            buff_full.scope,
            buff_mod,
            None,
            ModStrength::from_effect_mod_strength(buff_full.strength),
        )
    }
    pub(in crate::svc::calc) fn try_from_buff_with_attr(
        affector: RawModAffector,
        effect: &REffect,
        buff: &RBuff,
        buff_scope: REffectBuffScope,
        buff_mod: &RBuffModifier,
        buff_type_attr_rid: Option<RAttrId>,
        buff_str_attr_rid: RAttrId,
    ) -> Result<Option<Self>, RawModItemDependent> {
        Self::try_from_buff(
            affector,
            effect,
            buff,
            buff_scope,
            buff_mod,
            buff_type_attr_rid,
            ModStrength::Attr(buff_str_attr_rid),
        )
    }
    fn try_from_buff(
        affector: RawModAffector,
        effect: &REffect,
        buff: &RBuff,
        buff_scope: REffectBuffScope,
        buff_mod: &RBuffModifier,
        buff_type_attr_rid: Option<RAttrId>,
        buff_str: ModStrength,
    ) -> Result<Option<Self>, RawModItemDependent> {
        if !matches!(effect.category, AEffectCatId::ACTIVE | AEffectCatId::TARGET) {
            return Ok(None);
        }
        let affector_espec = affector.get_espec(effect);
        Ok(Some(match buff_scope {
            // Special processing for carrier scope. It is unknown how those self-buffs work on
            // non-ship items, since EVE does not have those in game, but we convert those into
            // local modifiers which affect just ship for simplicity of further processing
            REffectBuffScope::Carrier => Self {
                kind: ModifierKind::Local,
                affector_espec,
                strength: buff_str,
                op: CalcOp::from_a_op(buff.op),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::try_from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::Ship,
                    affector,
                )?,
                affectee_attr_rid: buff_mod.affectee_attr_rid,
                buff_type_attr_rid,
                ..
            },
            // Projected modifiers can be range-reduced and resisted
            REffectBuffScope::Projected(item_list_rid) => Self {
                kind: ModifierKind::Buff,
                affector_espec,
                strength: buff_str,
                op: CalcOp::from_a_op(buff.op),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::try_from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::ItemList(item_list_rid),
                    affector,
                )?,
                affectee_attr_rid: buff_mod.affectee_attr_rid,
                buff_type_attr_rid,
                proj_spec: effect.proj_mod,
                resist_attr_rid: get_resist_attr_rid(affector, effect)?,
                ..
            },
            // Fleet buffs cannot be resisted and range-reduced regardless of what effect says
            REffectBuffScope::Fleet(item_list_rid) => Self {
                kind: ModifierKind::FleetBuff,
                affector_espec,
                strength: buff_str,
                op: CalcOp::from_a_op(buff.op),
                aggr_mode: AggrMode::from_buff(buff),
                affectee_filter: AffecteeFilter::try_from_buff_affectee_filter(
                    &buff_mod.affectee_filter,
                    Location::ItemList(item_list_rid),
                    affector,
                )?,
                affectee_attr_rid: buff_mod.affectee_attr_rid,
                buff_type_attr_rid,
                ..
            },
        }))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Templates
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RawModifier {
    pub(super) fn with_affector_uid(&self, item_uid: UItemId) -> Self {
        let mut rmod = *self;
        rmod.affector_espec.item_uid = item_uid;
        rmod
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_resist_attr_rid(affector: RawModAffector, effect: &REffect) -> Result<Option<RAttrId>, RawModItemDependent> {
    match effect.proj_mod.and_then(|v| v.resist) {
        Some(resist) => affector.get_resist_attr_rid(resist),
        None => Ok(None),
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
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl RawModifier {
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                affector_espec,
                strength,
                affectee_attr_rid,
                buff_type_attr_rid,
                proj_spec,
                resist_attr_rid,
                // Not checked
                kind: _,
                op: _,
                aggr_mode: _,
                affectee_filter: _,
            } = self;
            affector_espec.consistency_check(u_data, true)?;
            strength.consistency_check(u_data)?;
            affectee_attr_rid.consistency_check(u_data)?;
            if let Some(attr_rid) = *buff_type_attr_rid {
                attr_rid.consistency_check(u_data)?;
            }
            if let Some(mspec) = proj_spec.as_ref() {
                mspec.consistency_check(u_data)?;
            }
            if let Some(attr_rid) = *resist_attr_rid {
                attr_rid.consistency_check(u_data)?;
            }
            Ok(())
        }
    }
}
