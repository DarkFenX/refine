use smallvec::{SmallVec, smallvec};

use super::custom::missile_flight_time;
use crate::{
    ad,
    sol::{
        AttrVal, ItemKey,
        svc::{
            EffectSpec, SvcCtx,
            calc::{AffectorInfo, Calc, CustomAffectorValue},
        },
        uad::item::UadItem,
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AffectorValue {
    AttrId(ad::AAttrId),
    Hardcoded(ad::AAttrVal),
    MissileFlightTime,
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
            Self::MissileFlightTime => None,
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
            Self::MissileFlightTime => missile_flight_time::get_affector_info(ctx, item_key),
            Self::Custom(custom) => (custom.affector_info_getter)(ctx, item_key),
        }
    }
    pub(super) fn get_mod_val(&self, calc: &mut Calc, ctx: &SvcCtx, espec: EffectSpec) -> Option<AttrVal> {
        match self {
            Self::AttrId(a_attr_id) => Some(calc.get_item_attr_val_full(ctx, espec.item_key, a_attr_id).ok()?.dogma),
            Self::Hardcoded(a_val) => Some(*a_val),
            Self::MissileFlightTime => missile_flight_time::get_mod_val(calc, ctx, espec),
            Self::Custom(custom) => (custom.mod_val_getter)(calc, ctx, espec),
        }
    }
    // Revision methods - define if modification value can change upon some action
    pub(super) fn revisable_on_item_add(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::MissileFlightTime => true,
            Self::Custom(custom) => custom.item_add_reviser.is_some(),
        }
    }
    pub(super) fn revisable_on_item_remove(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::MissileFlightTime => true,
            Self::Custom(custom) => custom.item_remove_reviser.is_some(),
        }
    }
    pub(super) fn revise_on_item_add(
        &self,
        ctx: &SvcCtx,
        affector_key: ItemKey,
        added_item_key: ItemKey,
        added_item: &UadItem,
    ) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::MissileFlightTime => missile_flight_time::revise_on_item_add_removal(ctx, affector_key, added_item),
            Self::Custom(custom) => custom.item_add_reviser.unwrap()(ctx, affector_key, added_item_key, added_item),
        }
    }
    pub(super) fn revise_on_item_remove(
        &self,
        ctx: &SvcCtx,
        affector_key: ItemKey,
        removed_item_key: ItemKey,
        removed_item: &UadItem,
    ) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::MissileFlightTime => missile_flight_time::revise_on_item_add_removal(ctx, affector_key, removed_item),
            Self::Custom(custom) => {
                custom.item_remove_reviser.unwrap()(ctx, affector_key, removed_item_key, removed_item)
            }
        }
    }
}
