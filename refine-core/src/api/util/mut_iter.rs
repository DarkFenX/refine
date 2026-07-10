//! Generic mutable iterator uses `lender` crate, which requires lends to check covariance. In other
//! lending iterators, it is checked via a macros it provides. Here, using macros is impossible,
//! since lends here use trait hierarchy, with some traits having an associated type. Associated
//! types on traits break covariance checks, so they are pushed further closer to specific types,
//! and lender itself just assumes covariance.
//!
//! tl;dr: a bunch of hacks to make it work. See https://github.com/WanderLanz/Lender/issues/37 for
//! more info.

use lender::{Lender, Lending, unsafe_assume_covariance};

use crate::{
    api::{
        AutochargeMut, BoosterMut, DroneMut, FighterMut, FitMut, FleetMut, FwEffectMut, ImplantMut, ProjEffectMut,
        RigMut, ServiceMut, SkillMut, SubsystemMut, SwEffectMut,
    },
    sol::SolarSystem,
    ud::{UFitId, UFleetId, UItemId},
};

pub struct MutIter<'this, T>
where
    T: New,
{
    sol: &'this mut SolarSystem,
    uids: Vec<T::UId>,
    index: usize,
    phantom: std::marker::PhantomData<T>,
}
impl<'this, T> MutIter<'this, T>
where
    T: New,
{
    pub(in crate::api) fn new(sol: &'this mut SolarSystem, uids: Vec<T::UId>) -> Self {
        Self {
            sol,
            uids,
            index: 0,
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'iter, 'lend, T> Lending<'lend> for MutIter<'iter, T>
where
    T: New,
{
    type Lend = <T as RefFamily>::Ref<'lend>;
}
impl<'iter, T> Lender for MutIter<'iter, T>
where
    T: New,
{
    unsafe_assume_covariance!();

    fn next(&mut self) -> Option<<T as RefFamily>::Ref<'_>> {
        let uid = *self.uids.get(self.index)?;
        self.index += 1;
        Some(T::new_new(self.sol, uid))
    }
}

pub trait RefFamily {
    type Ref<'a>;

    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>>;
}
pub(crate) trait New: RefFamily {
    type UId: Copy;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> <Self as RefFamily>::Ref<'_>;
}

// Implementations for non-item entities
impl RefFamily for FleetMut<'_> {
    type Ref<'a> = FleetMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for FleetMut<'_> {
    type UId = UFleetId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        FleetMut::__check_covariance(CovariantProof::new());
        FleetMut::new(sol, uid)
    }
}
impl RefFamily for FitMut<'_> {
    type Ref<'a> = FitMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for FitMut<'_> {
    type UId = UFitId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        FitMut::__check_covariance(CovariantProof::new());
        FitMut::new(sol, uid)
    }
}
// Implementations for items
impl RefFamily for AutochargeMut<'_> {
    type Ref<'a> = AutochargeMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for AutochargeMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        AutochargeMut::__check_covariance(CovariantProof::new());
        AutochargeMut::new(sol, uid)
    }
}
impl RefFamily for BoosterMut<'_> {
    type Ref<'a> = BoosterMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for BoosterMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        BoosterMut::__check_covariance(CovariantProof::new());
        BoosterMut::new(sol, uid)
    }
}
impl RefFamily for DroneMut<'_> {
    type Ref<'a> = DroneMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for DroneMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        DroneMut::__check_covariance(CovariantProof::new());
        DroneMut::new(sol, uid)
    }
}
impl RefFamily for FighterMut<'_> {
    type Ref<'a> = FighterMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for FighterMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        FighterMut::__check_covariance(CovariantProof::new());
        FighterMut::new(sol, uid)
    }
}
impl RefFamily for FwEffectMut<'_> {
    type Ref<'a> = FwEffectMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for FwEffectMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        FwEffectMut::__check_covariance(CovariantProof::new());
        FwEffectMut::new(sol, uid)
    }
}
impl RefFamily for ImplantMut<'_> {
    type Ref<'a> = ImplantMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for ImplantMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        ImplantMut::__check_covariance(CovariantProof::new());
        ImplantMut::new(sol, uid)
    }
}
impl RefFamily for ProjEffectMut<'_> {
    type Ref<'a> = ProjEffectMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for ProjEffectMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        ProjEffectMut::__check_covariance(CovariantProof::new());
        ProjEffectMut::new(sol, uid)
    }
}
impl RefFamily for RigMut<'_> {
    type Ref<'a> = RigMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for RigMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        RigMut::__check_covariance(CovariantProof::new());
        RigMut::new(sol, uid)
    }
}
impl RefFamily for ServiceMut<'_> {
    type Ref<'a> = ServiceMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for ServiceMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        ServiceMut::__check_covariance(CovariantProof::new());
        ServiceMut::new(sol, uid)
    }
}
impl RefFamily for SkillMut<'_> {
    type Ref<'a> = SkillMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for SkillMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        SkillMut::__check_covariance(CovariantProof::new());
        SkillMut::new(sol, uid)
    }
}
impl RefFamily for SubsystemMut<'_> {
    type Ref<'a> = SubsystemMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for SubsystemMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        SubsystemMut::__check_covariance(CovariantProof::new());
        SubsystemMut::new(sol, uid)
    }
}
impl RefFamily for SwEffectMut<'_> {
    type Ref<'a> = SwEffectMut<'a>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Ref<'long>>,
    ) -> CovariantProof<Self::Ref<'short>> {
        proof
    }
}
impl New for SwEffectMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        SwEffectMut::__check_covariance(CovariantProof::new());
        SwEffectMut::new(sol, uid)
    }
}

// Covariance check-specific things
pub struct CovariantProof<T>(core::marker::PhantomData<fn() -> T>);
impl<T> CovariantProof<T> {
    fn new() -> Self {
        CovariantProof(core::marker::PhantomData)
    }
}
