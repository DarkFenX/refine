use smallvec::{SmallVec, smallvec};

use crate::{
    api::AttrId,
    misc::{EffectSpec, Value},
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::{Affector, Calc, CustomAffectorValue, ItemAddReviser, ItemRemoveReviser},
    },
    ud::UItemId,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AffectorValue {
    Attr(RAttrId),
    Hardcoded(Value),
    Custom(CustomAffectorValue),
}
impl AffectorValue {
    // Simple and fast way to get affector attribute. Variants which have actual affector attributes
    // but do not expose anything are designed to handle attribute cleanup in some other way (via
    // dependency/revision registers)
    pub(super) fn get_affector_attr_rid(&self) -> Option<RAttrId> {
        match self {
            Self::Attr(attr_rid) => Some(*attr_rid),
            Self::Hardcoded(_) => None,
            Self::Custom(custom) => custom.affector_attr_rid,
        }
    }
    // More expensive, but comprehensive info about affecting items/attributes
    pub(super) fn get_affector_info(&self, ctx: SvcCtx, item_uid: UItemId) -> SmallVec<Affector, 1> {
        match self {
            Self::Attr(attr_rid) => smallvec![Affector {
                item_id: ctx.u_data.items.xid_by_iid(item_uid),
                attr_id: Some(AttrId::from_aid(ctx.u_data.src.get_attr_by_rid(*attr_rid).aid)),
            }],
            Self::Hardcoded(_) => smallvec![Affector {
                item_id: ctx.u_data.items.xid_by_iid(item_uid),
                attr_id: None
            }],
            Self::Custom(custom) => (custom.affector_info_getter)(ctx, item_uid),
        }
    }
    pub(super) fn get_mod_val(&self, calc: &mut Calc, ctx: SvcCtx, espec: EffectSpec) -> Option<Value> {
        match self {
            Self::Attr(attr_rid) => Some(calc.get_item_attr_rfull(ctx, espec.item_uid, *attr_rid).ok()?.dogma),
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
