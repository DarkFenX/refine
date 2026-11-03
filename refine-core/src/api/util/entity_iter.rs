//! Iterator over entities (fleets, fits, items) uses `lender` crate, which requires lends to check
//! covariance. In other lending iterators, it is checked via a macros it provides. Here, using
//! macros is impossible, since lends here use trait hierarchy, with some traits having an
//! associated type. Associated types on traits break covariance checks, so they are pushed further
//! closer to specific types, and lender itself just assumes covariance.
//!
//! tl;dr: a bunch of hacks to make it work. See <https://github.com/WanderLanz/Lender/issues/37>
//! for more info.

use lender::{ExactSizeLender, Lender, Lending, unsafe_assume_covariance};

use crate::{
    Autocharge, Booster, Drone, Fighter, Fit, Fleet, FwEffect, Implant, Item, ProjEffect, Rig, Service, Skill,
    SolarSystem, Subsystem, SwEffect,
    ud::{UFitId, UFleetId, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
#[expect(private_bounds)]
pub struct EntityIter<'iter, T>
where
    T: FromUid,
{
    sol: &'iter mut SolarSystem,
    uids: Vec<T::UId>,
    index: usize,
    phantom: std::marker::PhantomData<T>,
}
impl<'iter, 'lend, T> Lending<'lend> for EntityIter<'iter, T>
where
    T: FromUid,
{
    type Lend = <T as EntityFamily>::Handle<'lend>;
}
impl<'iter, T> Lender for EntityIter<'iter, T>
where
    T: FromUid,
{
    unsafe_assume_covariance!();

    fn next(&mut self) -> Option<<T as EntityFamily>::Handle<'_>> {
        let uid = *self.uids.get(self.index)?;
        self.index += 1;
        Some(T::from_uid(self.sol, uid))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.uids.len() - self.index;
        (remaining, Some(remaining))
    }
}
impl<'iter, T> ExactSizeLender for EntityIter<'iter, T> where T: FromUid {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
#[expect(private_bounds)]
impl<'iter, T> EntityIter<'iter, T>
where
    T: FromUid,
{
    pub(in crate::api) fn new(sol: &'iter mut SolarSystem, uids: Vec<T::UId>) -> Self {
        Self {
            sol,
            uids,
            index: 0,
            phantom: std::marker::PhantomData,
        }
    }
}

pub trait EntityFamily {
    type Handle<'a>;

    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>>;
}
pub(crate) trait FromUid: EntityFamily {
    type UId: Copy;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> <Self as EntityFamily>::Handle<'_>;
}

// Implementations for non-item entities
impl EntityFamily for Fleet<'_> {
    type Handle<'s> = Fleet<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Fleet<'_> {
    type UId = UFleetId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Fleet::__check_covariance(CovariantProof::new());
        Fleet::new(sol, uid)
    }
}
impl EntityFamily for Fit<'_> {
    type Handle<'s> = Fit<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Fit<'_> {
    type UId = UFitId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Fit::__check_covariance(CovariantProof::new());
        Fit::new(sol, uid)
    }
}
// Implementations for items
impl EntityFamily for Item<'_> {
    type Handle<'s> = Item<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Item<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Item::__check_covariance(CovariantProof::new());
        Item::new(sol, uid)
    }
}
impl EntityFamily for Autocharge<'_> {
    type Handle<'s> = Autocharge<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Autocharge<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Autocharge::__check_covariance(CovariantProof::new());
        Autocharge::new(sol, uid)
    }
}
impl EntityFamily for Booster<'_> {
    type Handle<'s> = Booster<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Booster<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Booster::__check_covariance(CovariantProof::new());
        Booster::new(sol, uid)
    }
}
impl EntityFamily for Drone<'_> {
    type Handle<'s> = Drone<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Drone<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Drone::__check_covariance(CovariantProof::new());
        Drone::new(sol, uid)
    }
}
impl EntityFamily for Fighter<'_> {
    type Handle<'s> = Fighter<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Fighter<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Fighter::__check_covariance(CovariantProof::new());
        Fighter::new(sol, uid)
    }
}
impl EntityFamily for FwEffect<'_> {
    type Handle<'s> = FwEffect<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for FwEffect<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        FwEffect::__check_covariance(CovariantProof::new());
        FwEffect::new(sol, uid)
    }
}
impl EntityFamily for Implant<'_> {
    type Handle<'s> = Implant<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Implant<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Implant::__check_covariance(CovariantProof::new());
        Implant::new(sol, uid)
    }
}
impl EntityFamily for ProjEffect<'_> {
    type Handle<'s> = ProjEffect<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for ProjEffect<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        ProjEffect::__check_covariance(CovariantProof::new());
        ProjEffect::new(sol, uid)
    }
}
impl EntityFamily for Rig<'_> {
    type Handle<'s> = Rig<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Rig<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Rig::__check_covariance(CovariantProof::new());
        Rig::new(sol, uid)
    }
}
impl EntityFamily for Service<'_> {
    type Handle<'s> = Service<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Service<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Service::__check_covariance(CovariantProof::new());
        Service::new(sol, uid)
    }
}
impl EntityFamily for Skill<'_> {
    type Handle<'s> = Skill<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Skill<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Skill::__check_covariance(CovariantProof::new());
        Skill::new(sol, uid)
    }
}
impl EntityFamily for Subsystem<'_> {
    type Handle<'s> = Subsystem<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Subsystem<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        Subsystem::__check_covariance(CovariantProof::new());
        Subsystem::new(sol, uid)
    }
}
impl EntityFamily for SwEffect<'_> {
    type Handle<'s> = SwEffect<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for SwEffect<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolarSystem, uid: Self::UId) -> Self::Handle<'_> {
        SwEffect::__check_covariance(CovariantProof::new());
        SwEffect::new(sol, uid)
    }
}

// Covariance check-specific things
pub struct CovariantProof<T>(core::marker::PhantomData<fn() -> T>);
impl<T> CovariantProof<T> {
    fn new() -> Self {
        CovariantProof(core::marker::PhantomData)
    }
}
