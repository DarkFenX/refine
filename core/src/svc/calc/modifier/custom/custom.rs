use std::hash::{Hash, Hasher};

use super::{
    aar_paste::add_aar_paste_mod, missile_flight_time::add_missile_flight_time_mod, prop_speed::add_prop_speed_mod,
};
use crate::{
    misc::EffectSpec,
    rd::{RAttrConsts, RAttrId},
    svc::calc::RawModifier,
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
            Self::MissileFlightTime => add_missile_flight_time_mod(rmods, attr_consts, affector_espec),
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
