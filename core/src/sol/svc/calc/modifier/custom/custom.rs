use smallvec::SmallVec;

use crate::{
    ad,
    sol::{
        AttrVal, ItemKey,
        svc::{
            EffectSpec, SvcCtx,
            calc::{AffectorInfo, Calc},
        },
        uad::item::UadItem,
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CustomAffectorValue {
    // Modifiers have two ways to define affector attribute:
    // - cheap way is via this field, with limitation that value of the attribute has to be on the
    //   same item as the effect modifier is created from. All the regular modifiers use this
    //   approach;
    // - more expensive and flexible way via registering arbitrary attribute dependencies in the
    //  dependency register during attribute value calculation.
    // Use this field over the dependency approach whenever possible.
    pub(crate) affector_a_attr_id: Option<ad::AAttrId>,
    // Should return all the affecting attributes. Can be slow, used only when fetching modification
    // info
    pub(crate) affector_info_getter: fn(&SvcCtx, ItemKey) -> SmallVec<AffectorInfo, 1>,
    pub(crate) mod_val_getter: fn(&mut Calc, &SvcCtx, EffectSpec) -> Option<AttrVal>,
    // Reviser functions are triggered upon certain events; if they return true, affected attribute
    // values are marked for recalculation.
    pub(crate) item_add_reviser: Option<fn(&SvcCtx, ItemKey, ItemKey, &UadItem) -> bool> = None,
    pub(crate) item_remove_reviser: Option<fn(&SvcCtx, ItemKey, ItemKey, &UadItem) -> bool> = None,
}
