//! Iterator over entities (fleets, fits, items) uses `lender` crate, which requires lends to check
//! covariance. In other lending iterators, it is checked via a macro it provides. Here, the macro
//! can't check it, since lends here go through an associated type of a generic parameter, so each
//! entity proves it for its own handle, and the lender forwards to that proof.
//!
//! See <https://github.com/WanderLanz/Lender/issues/37> for more info.

use lender::{__check_lender_covariance, CovariantProof, ExactSizeLender, Lender, Lending};

use crate::{
    Autocharge, Booster, Drone, Fighter, Fit, Fleet, FwEffect, Implant, Item, Module, ProjEffect, Rig, Service, Skill,
    Subsystem, SwEffect,
    sol::SolCore,
    stats::{FitStats, FleetStats, ItemStats},
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
    sol: &'iter mut SolCore,
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
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<<Self as Lending<'long>>::Lend>,
    ) -> CovariantProof<<Self as Lending<'short>>::Lend> {
        T::__check_covariance(proof)
    }

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
    pub(in crate::api) fn new(sol: &'iter mut SolCore, uids: Vec<T::UId>) -> Self {
        __check_lender_covariance::<Self>();
        Self {
            sol,
            uids,
            index: 0,
            phantom: std::marker::PhantomData,
        }
    }
}

#[doc(hidden)]
pub trait EntityFamily {
    type Handle<'a>;

    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>>;
}
pub(crate) trait FromUid: EntityFamily {
    type UId: Copy;
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> <Self as EntityFamily>::Handle<'_>;
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
        Implant::new(sol, uid)
    }
}
impl EntityFamily for Module<'_> {
    type Handle<'s> = Module<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for Module<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
        Module::new(sol, uid)
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
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
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
        SwEffect::new(sol, uid)
    }
}
// Implementations for stats
impl EntityFamily for FleetStats<'_> {
    type Handle<'s> = FleetStats<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for FleetStats<'_> {
    type UId = UFleetId;
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
        FleetStats::new(sol, uid)
    }
}
impl EntityFamily for FitStats<'_> {
    type Handle<'s> = FitStats<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for FitStats<'_> {
    type UId = UFitId;
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
        FitStats::new(sol, uid)
    }
}
impl EntityFamily for ItemStats<'_> {
    type Handle<'s> = ItemStats<'s>;
    #[inline(always)]
    fn __check_covariance<'long: 'short, 'short>(
        proof: CovariantProof<Self::Handle<'long>>,
    ) -> CovariantProof<Self::Handle<'short>> {
        proof
    }
}
impl FromUid for ItemStats<'_> {
    type UId = UItemId;
    fn from_uid(sol: &mut SolCore, uid: Self::UId) -> Self::Handle<'_> {
        ItemStats::new(sol, uid)
    }
}
