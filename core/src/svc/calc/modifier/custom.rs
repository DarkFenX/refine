use std::hash::{Hash, Hasher};

use smallvec::SmallVec;

use crate::{
    def::AttrVal,
    misc::EffectSpec,
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::{Affector, Calc},
    },
    ud::{UItem, UItemId},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum CustomAffectorValueKind {
    PropSpeedBoost,
    AarRepAmount,
    MissileFlightTime,
}

pub(crate) type ItemAddReviser = fn(SvcCtx, UItemId, UItemId, &UItem) -> bool;
pub(crate) type ItemRemoveReviser = fn(SvcCtx, UItemId, UItemId, &UItem) -> bool;

#[derive(Copy, Clone)]
pub(crate) struct CustomAffectorValue {
    // Field to use for hashing/comparison, not to rely on function pointers
    pub(crate) kind: CustomAffectorValueKind,
    // Modifiers have two ways to define affector attribute:
    // - cheap way is via this field, with limitation that value of the attribute has to be on the
    //   same item as the effect modifier is created from. All the regular modifiers use this
    //   approach;
    // - more expensive and flexible way via registering arbitrary attribute dependencies in the
    //  dependency register during attribute value calculation.
    // Use this field over the dependency approach whenever possible.
    pub(crate) affector_attr_key: Option<RAttrId>,
    // Should return all the affecting attributes. Can be slow, used only when fetching modification
    // info
    pub(crate) affector_info_getter: fn(SvcCtx, UItemId) -> SmallVec<Affector, 1>,
    pub(crate) mod_val_getter: fn(&mut Calc, SvcCtx, EffectSpec) -> Option<AttrVal>,
    // Reviser functions are triggered upon certain events; if they return true, affected attribute
    // values are marked for recalculation.
    pub(crate) item_add_reviser: Option<ItemAddReviser> = None,
    pub(crate) item_remove_reviser: Option<ItemRemoveReviser> = None,
}
impl PartialEq for CustomAffectorValue {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}
impl Eq for CustomAffectorValue {}
impl Hash for CustomAffectorValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}
