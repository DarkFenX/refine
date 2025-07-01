use smallvec::{SmallVec, smallvec};

use crate::{
    ad,
    def::{AttrVal, ItemKey},
    misc::EffectSpec,
    svc::{
        SvcCtx,
        calc::{AffectorInfo, Calc, CustomAffectorValue, ItemAddReviser, ItemRemoveReviser},
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AffectorValue {
    AttrId(ad::AAttrId),
    Hardcoded(ad::AAttrVal),
    Custom(CustomAffectorValue),
}
impl AffectorValue {
    // Simple and fast way to get affector attribute. Variants which have actual affector attributes
    // but do not expose anything are designed to handle attribute cleanup in some other way (via
    // dependency/revision registers)
    pub(super) fn get_affector_a_attr_id(&self) -> Option<ad::AAttrId> {
        match self {
            Self::AttrId(attr_id) => Some(*attr_id),
            Self::Hardcoded(_) => None,
            Self::Custom(custom) => custom.affector_a_attr_id,
        }
    }
    // More expensive, but comprehensive info about affecting items/attributes
    pub(super) fn get_affector_info(&self, ctx: &SvcCtx, item_key: ItemKey) -> SmallVec<AffectorInfo, 1> {
        match self {
            Self::AttrId(attr_id) => smallvec![AffectorInfo {
                item_id: ctx.uad.items.id_by_key(item_key),
                attr_id: Some(*attr_id)
            }],
            Self::Hardcoded(_) => smallvec![AffectorInfo {
                item_id: ctx.uad.items.id_by_key(item_key),
                attr_id: None
            }],
            Self::Custom(custom) => (custom.affector_info_getter)(ctx, item_key),
        }
    }
    pub(super) fn get_mod_val(&self, calc: &mut Calc, ctx: &SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
        match self {
            Self::AttrId(a_attr_id) => Some(calc.get_item_attr_val_full(ctx, espec.item_key, a_attr_id).ok()?.dogma),
            Self::Hardcoded(a_val) => Some(*a_val),
            Self::Custom(custom) => (custom.mod_val_getter)(calc, ctx, espec),
        }
    }
    // Revision methods - define if modification value can change upon some action
    pub(super) fn get_item_add_reviser(&self) -> Option<ItemAddReviser> {
        match self {
            Self::AttrId(_) => None,
            Self::Hardcoded(_) => None,
            Self::Custom(custom) => custom.item_add_reviser,
        }
    }
    pub(super) fn get_item_remove_reviser(&self) -> Option<ItemRemoveReviser> {
        match self {
            Self::AttrId(_) => None,
            Self::Hardcoded(_) => None,
            Self::Custom(custom) => custom.item_remove_reviser,
        }
    }
}
