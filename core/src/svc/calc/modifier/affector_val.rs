use smallvec::{SmallVec, smallvec};

use crate::{
    ad::AAttrVal,
    def::AttrVal,
    misc::EffectSpec,
    rd::RAttrKey,
    svc::{
        SvcCtx,
        calc::{Affector, Calc, CustomAffectorValue, ItemAddReviser, ItemRemoveReviser},
    },
    ud::UItemKey,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AffectorValue {
    Attr(RAttrKey),
    Hardcoded(AAttrVal),
    Custom(CustomAffectorValue),
}
impl AffectorValue {
    // Simple and fast way to get affector attribute. Variants which have actual affector attributes
    // but do not expose anything are designed to handle attribute cleanup in some other way (via
    // dependency/revision registers)
    pub(super) fn get_affector_attr_key(&self) -> Option<RAttrKey> {
        match self {
            Self::Attr(attr_key) => Some(*attr_key),
            Self::Hardcoded(_) => None,
            Self::Custom(custom) => custom.affector_attr_key,
        }
    }
    // More expensive, but comprehensive info about affecting items/attributes
    pub(super) fn get_affector_info(&self, ctx: SvcCtx, item_key: UItemKey) -> SmallVec<Affector, 1> {
        match self {
            Self::Attr(attr_key) => smallvec![Affector {
                item_id: ctx.u_data.items.id_by_key(item_key),
                attr_id: Some(ctx.u_data.src.get_attr(*attr_key).id.into()),
            }],
            Self::Hardcoded(_) => smallvec![Affector {
                item_id: ctx.u_data.items.id_by_key(item_key),
                attr_id: None
            }],
            Self::Custom(custom) => (custom.affector_info_getter)(ctx, item_key),
        }
    }
    pub(super) fn get_mod_val(&self, calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
        match self {
            Self::Attr(attr_key) => Some(calc.get_item_attr_rfull(ctx, espec.item_key, *attr_key).ok()?.dogma),
            Self::Hardcoded(a_val) => Some(*a_val),
            Self::Custom(custom) => (custom.mod_val_getter)(calc, ctx, espec),
        }
    }
    // Revision methods - define if modification value can change upon some action
    pub(super) fn get_item_add_reviser(&self) -> Option<ItemAddReviser> {
        match self {
            Self::Attr(_) => None,
            Self::Hardcoded(_) => None,
            Self::Custom(custom) => custom.item_add_reviser,
        }
    }
    pub(super) fn get_item_remove_reviser(&self) -> Option<ItemRemoveReviser> {
        match self {
            Self::Attr(_) => None,
            Self::Hardcoded(_) => None,
            Self::Custom(custom) => custom.item_remove_reviser,
        }
    }
}
