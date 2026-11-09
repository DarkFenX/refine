use smallvec::SmallVec;

use super::{aar_paste, missile_flight_time, prop_speed, reviser::ItemAddRemoveReviser};
use crate::{
    Value,
    misc::EffectSpec,
    rd::{RAttrConsts, RAttrId, REffect},
    svc::{
        SvcCtx,
        calc::{Calc, CalcModInfoAffector, RawModAffector, RawModItemDependent, RawModifier},
    },
    ud::UItemId,
};

#[cfg_attr(feature = "dev", derive(PartialEq))]
#[derive(Copy, Clone)]
pub(crate) enum CalcCustomModifier {
    PropSpeed,
    AarPaste,
    MissileFlightTime,
}
impl CalcCustomModifier {
    pub(in crate::svc::calc) fn make_rmod(
        self,
        attr_consts: &RAttrConsts,
        affector: RawModAffector,
        effect: &REffect,
    ) -> Result<Option<RawModifier>, RawModItemDependent> {
        // None of custom modifiers are source-item-dependent so far
        Ok(match self {
            Self::PropSpeed => prop_speed::make_rmod(attr_consts, affector, effect),
            Self::AarPaste => aar_paste::make_rmod(attr_consts, affector, effect),
            Self::MissileFlightTime => missile_flight_time::make_rmod(attr_consts, affector, effect),
        })
    }
}

#[cfg_attr(feature = "dev", derive(PartialEq))]
#[derive(Copy, Clone)]
pub(in crate::svc::calc::modifier) struct CalcCustomModStrength {
    pub(super) kind: CalcCustomModifier,
    /// Modifiers have two ways to define affector attribute:
    /// - cheap way is via this field, with limitation that value of the attribute has to be on the
    ///   same item as the effect modifier is created from. All the regular modifiers use this
    ///   approach;
    /// - more expensive and flexible way via registering arbitrary attribute dependencies in the
    ///   dependency register during attribute value calculation.
    ///
    /// Use this field over the dependency approach whenever possible.
    pub(in crate::svc::calc::modifier) affector_attr_rid: Option<RAttrId>,
}
impl CalcCustomModStrength {
    pub(in crate::svc::calc::modifier) fn get_affector_info(
        self,
        ctx: &SvcCtx,
        item_uid: UItemId,
    ) -> SmallVec<[CalcModInfoAffector; 1]> {
        match &self.kind {
            CalcCustomModifier::PropSpeed => prop_speed::get_affector_info(ctx, item_uid),
            CalcCustomModifier::AarPaste => aar_paste::get_affector_info(ctx, item_uid),
            CalcCustomModifier::MissileFlightTime => missile_flight_time::get_affector_info(ctx, item_uid),
        }
    }
    pub(in crate::svc::calc::modifier) fn get_strength(
        self,
        calc: &mut Calc,
        ctx: &SvcCtx,
        espec: EffectSpec,
    ) -> Option<Value> {
        match &self.kind {
            CalcCustomModifier::PropSpeed => prop_speed::get_mod_val(calc, ctx, espec),
            CalcCustomModifier::AarPaste => aar_paste::get_mod_val(calc, ctx, espec),
            CalcCustomModifier::MissileFlightTime => missile_flight_time::get_mod_val(calc, ctx, espec),
        }
    }
    pub(in crate::svc::calc::modifier) fn get_item_add_remove_reviser(self) -> Option<ItemAddRemoveReviser> {
        match &self.kind {
            CalcCustomModifier::PropSpeed => None,
            CalcCustomModifier::AarPaste => Some(ItemAddRemoveReviser::AarPaste),
            CalcCustomModifier::MissileFlightTime => Some(ItemAddRemoveReviser::MissileFlightTime),
        }
    }
    pub(in crate::svc::calc::modifier) fn adds_deps_with_source(self) -> bool {
        match &self.kind {
            CalcCustomModifier::PropSpeed => true,
            CalcCustomModifier::AarPaste => false,
            CalcCustomModifier::MissileFlightTime => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl CalcCustomModStrength {
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                affector_attr_rid,
                // Not checked
                kind: _,
            } = self;
            if let Some(attr_rid) = affector_attr_rid {
                attr_rid.consistency_check(u_data)?;
            }
            Ok(())
        }
    }
}
