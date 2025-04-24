use lender::{Lender, Lending};

use crate::sol::{
    FitKey, FleetKey, ItemKey, SolarSystem,
    api::{
        AutochargeMut, BoosterMut, DroneMut, FighterMut, FitMut, FleetMut, FwEffectMut, ImplantMut, ProjEffectMut,
        RigMut, ServiceMut, SkillMut, SubsystemMut, SwEffectMut,
    },
};

pub struct MutIter<'this, T>
where
    T: New,
{
    sol: &'this mut SolarSystem,
    keys: Vec<T::Key>,
    index: usize,
    phantom: std::marker::PhantomData<T>,
}
impl<'this, T> MutIter<'this, T>
where
    T: New,
{
    pub(in crate::sol::api) fn new(sol: &'this mut SolarSystem, keys: Vec<T::Key>) -> Self {
        Self {
            sol,
            keys,
            index: 0,
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'iter, 'lend, T> Lending<'lend> for MutIter<'iter, T>
where
    T: New,
{
    type Lend = <T as New>::This<'lend>;
}
impl<'iter, T> Lender for MutIter<'iter, T>
where
    T: New,
{
    fn next(&mut self) -> Option<<T as New>::This<'_>> {
        let key = *self.keys.get(self.index)?;
        self.index += 1;
        Some(T::new_new(self.sol, key))
    }
}

pub trait New {
    type Key: Copy + Clone;
    type This<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_>;
}
// Implementations for non-item entities
pub enum FleetMutGenerator {}
impl New for FleetMutGenerator {
    type Key = FleetKey;
    type This<'a> = FleetMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        FleetMut::new(sol, key)
    }
}
pub enum FitMutGenerator {}
impl New for FitMutGenerator {
    type Key = FitKey;
    type This<'a> = FitMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        FitMut::new(sol, key)
    }
}
// Implementations for items
pub enum AutochargeMutGenerator {}
impl New for AutochargeMutGenerator {
    type Key = ItemKey;
    type This<'a> = AutochargeMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        AutochargeMut::new(sol, key)
    }
}
pub enum BoosterMutGenerator {}
impl New for BoosterMutGenerator {
    type Key = ItemKey;
    type This<'a> = BoosterMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        BoosterMut::new(sol, key)
    }
}
pub enum DroneMutGenerator {}
impl New for DroneMutGenerator {
    type Key = ItemKey;
    type This<'a> = DroneMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        DroneMut::new(sol, key)
    }
}
pub enum FighterMutGenerator {}
impl New for FighterMutGenerator {
    type Key = ItemKey;
    type This<'a> = FighterMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        FighterMut::new(sol, key)
    }
}
pub enum FwEffectMutGenerator {}
impl New for FwEffectMutGenerator {
    type Key = ItemKey;
    type This<'a> = FwEffectMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        FwEffectMut::new(sol, key)
    }
}
pub enum ImplantMutGenerator {}
impl New for ImplantMutGenerator {
    type Key = ItemKey;
    type This<'a> = ImplantMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        ImplantMut::new(sol, key)
    }
}
pub enum ProjEffectMutGenerator {}
impl New for ProjEffectMutGenerator {
    type Key = ItemKey;
    type This<'a> = ProjEffectMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        ProjEffectMut::new(sol, key)
    }
}
pub enum RigMutGenerator {}
impl New for RigMutGenerator {
    type Key = ItemKey;
    type This<'a> = RigMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        RigMut::new(sol, key)
    }
}
pub enum ServiceMutGenerator {}
impl New for ServiceMutGenerator {
    type Key = ItemKey;
    type This<'a> = ServiceMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        ServiceMut::new(sol, key)
    }
}
pub enum SkillMutGenerator {}
impl New for SkillMutGenerator {
    type Key = ItemKey;
    type This<'a> = SkillMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        SkillMut::new(sol, key)
    }
}
pub enum SubsystemMutGenerator {}
impl New for SubsystemMutGenerator {
    type Key = ItemKey;
    type This<'a> = SubsystemMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        SubsystemMut::new(sol, key)
    }
}
pub enum SwEffectMutGenerator {}
impl New for SwEffectMutGenerator {
    type Key = ItemKey;
    type This<'a> = SwEffectMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        SwEffectMut::new(sol, key)
    }
}
