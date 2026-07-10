use smallvec::{SmallVec, smallvec};

use super::custom::CalcCustomModStrength;
use crate::{
    dbg::DebugResult,
    misc::EffectSpec,
    num::Value,
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::{Calc, CalcModInfoAffector, ItemAddRemoveReviser},
    },
    ud::{UData, UItemId},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum ModStrength {
    Attr(RAttrId),
    Hardcoded(Value),
    Custom(CalcCustomModStrength),
}
impl ModStrength {
    // Simple and fast way to get affector attribute. Variants which have actual affector attributes
    // but do not expose anything are designed to handle attribute cleanup in some other way (via
    // dependency/revision registers)
    pub(super) fn get_affector_attr_rid(&self) -> Option<RAttrId> {
        match self {
            Self::Attr(attr_rid) => Some(*attr_rid),
            Self::Hardcoded(_) => None,
            Self::Custom(custom_str) => custom_str.affector_attr_rid,
        }
    }
    // More expensive, but comprehensive info about affecting items/attributes
    pub(super) fn get_affector_info(&self, ctx: SvcCtx, item_uid: UItemId) -> SmallVec<[CalcModInfoAffector; 1]> {
        match self {
            Self::Attr(attr_rid) => smallvec![CalcModInfoAffector {
                item_uid,
                attr_rid: Some(*attr_rid),
            }],
            Self::Hardcoded(_) => smallvec![CalcModInfoAffector {
                item_uid,
                attr_rid: None
            }],
            Self::Custom(custom_str) => custom_str.get_affector_info(ctx, item_uid),
        }
    }
    pub(super) fn get_strength(&self, calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<Value> {
        match self {
            Self::Attr(attr_rid) => Some(calc.get_item_attr_rfull(ctx, espec.item_uid, *attr_rid).ok()?.dogma),
            Self::Hardcoded(strength) => Some(*strength),
            Self::Custom(custom_str) => custom_str.get_strength(calc, ctx, espec),
        }
    }
    // Revision methods - define if modification value can change upon some action
    pub(super) fn get_item_add_remove_reviser(&self) -> Option<ItemAddRemoveReviser> {
        match self {
            Self::Attr(_) => None,
            Self::Hardcoded(_) => None,
            Self::Custom(custom_str) => custom_str.get_item_add_remove_reviser(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ModStrength {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        match self {
            Self::Attr(attr_rid) => attr_rid.consistency_check(u_data)?,
            Self::Hardcoded(_) => (),
            Self::Custom(custom_str) => custom_str.consistency_check(u_data)?,
        }
        Ok(())
    }
}
