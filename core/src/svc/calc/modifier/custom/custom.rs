use std::hash::{Hash, Hasher};

use smallvec::SmallVec;

use super::{aar_paste::add_aar_paste_mod, prop_speed::add_prop_speed_mod, reviser::CustomModReviser};
use crate::{
    misc::EffectSpec,
    num::Value,
    rd::{RAttrConsts, RAttrId},
    svc::{
        SvcCtx,
        calc::{Affector, Calc, RawModifier},
    },
    ud::{UItem, UItemId},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum CalcCustomModifier {
    PropSpeed,
    AarPaste,
    MissileFlightTime,
}
impl CalcCustomModifier {
    pub(in crate::svc::calc) fn add(
        &self,
        rmods: &mut Vec<RawModifier>,
        attr_consts: &RAttrConsts,
        affector_espec: EffectSpec,
    ) {
        match self {
            Self::PropSpeed => add_prop_speed_mod(rmods, attr_consts, affector_espec),
            Self::AarPaste => add_aar_paste_mod(rmods, attr_consts, affector_espec),
            Self::MissileFlightTime => (),
        }
    }
}

#[derive(Copy, Clone)]
pub(in crate::svc::calc::modifier) struct CalcCustomAffectorValue {
    pub(in crate::svc::calc::modifier) kind: CalcCustomModifier,
    // Modifiers have two ways to define affector attribute:
    // - cheap way is via this field, with limitation that value of the attribute has to be on the same item as the
    //   effect modifier is created from. All the regular modifiers use this approach;
    // - more expensive and flexible way via registering arbitrary attribute dependencies in the
    //  dependency register during attribute value calculation.
    // Use this field over the dependency approach whenever possible.
    pub(in crate::svc::calc::modifier) affector_attr_rid: Option<RAttrId>,
}
impl PartialEq for CalcCustomAffectorValue {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}
impl Eq for CalcCustomAffectorValue {}
impl Hash for CalcCustomAffectorValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}
impl CalcCustomAffectorValue {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TODO: old stuff ahead, to remove
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum CustomAffectorValueKind {
    PropSpeedBoost,
    AarRepAmount,
    MissileFlightTime,
}

type ItemAddReviser = fn(SvcCtx, UItemId, UItemId, &UItem) -> bool;
type ItemRemoveReviser = fn(SvcCtx, UItemId, UItemId, &UItem) -> bool;

#[derive(Copy, Clone)]
struct CustomAffectorValue {
    // Field to use for hashing/comparison, not to rely on function pointers
     kind: CustomAffectorValueKind,
    // Modifiers have two ways to define affector attribute:
    // - cheap way is via this field, with limitation that value of the attribute has to be on the
    //   same item as the effect modifier is created from. All the regular modifiers use this
    //   approach;
    // - more expensive and flexible way via registering arbitrary attribute dependencies in the
    //  dependency register during attribute value calculation.
    // Use this field over the dependency approach whenever possible.
     affector_attr_rid: Option<RAttrId>,
    // Should return all the affecting attributes. Can be slow, used only when fetching modification
    // info
     affector_info_getter: fn(SvcCtx, UItemId) -> SmallVec<Affector, 1>,
     mod_val_getter: fn(&mut Calc, SvcCtx, EffectSpec) -> Option<Value>,
    // Reviser functions are triggered upon certain events; if they return true, affected attribute
    // values are marked for recalculation.
     item_add_reviser: Option<ItemAddReviser> = None,
     item_remove_reviser: Option<ItemRemoveReviser> = None,
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
