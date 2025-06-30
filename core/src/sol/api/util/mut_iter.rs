use lender::{Lender, Lending};

use crate::{
    def::{FitKey, FleetKey, ItemKey},
    sol::{
        SolarSystem,
        api::{
            AutochargeMut, BoosterMut, DroneMut, FighterMut, FitMut, FleetMut, FwEffectMut, ImplantMut, ProjEffectMut,
            RigMut, ServiceMut, SkillMut, SubsystemMut, SwEffectMut,
        },
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
impl New for FleetMut<'_> {
    type Key = FleetKey;
    type This<'a> = FleetMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        FleetMut::new(sol, key)
    }
}
impl New for FitMut<'_> {
    type Key = FitKey;
    type This<'a> = FitMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        FitMut::new(sol, key)
    }
}
// Implementations for items
impl New for AutochargeMut<'_> {
    type Key = ItemKey;
    type This<'a> = AutochargeMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        AutochargeMut::new(sol, key)
    }
}
impl New for BoosterMut<'_> {
    type Key = ItemKey;
    type This<'a> = BoosterMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        BoosterMut::new(sol, key)
    }
}
impl New for DroneMut<'_> {
    type Key = ItemKey;
    type This<'a> = DroneMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        DroneMut::new(sol, key)
    }
}
impl New for FighterMut<'_> {
    type Key = ItemKey;
    type This<'a> = FighterMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        FighterMut::new(sol, key)
    }
}
impl New for FwEffectMut<'_> {
    type Key = ItemKey;
    type This<'a> = FwEffectMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        FwEffectMut::new(sol, key)
    }
}
impl New for ImplantMut<'_> {
    type Key = ItemKey;
    type This<'a> = ImplantMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        ImplantMut::new(sol, key)
    }
}
impl New for ProjEffectMut<'_> {
    type Key = ItemKey;
    type This<'a> = ProjEffectMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        ProjEffectMut::new(sol, key)
    }
}
impl New for RigMut<'_> {
    type Key = ItemKey;
    type This<'a> = RigMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        RigMut::new(sol, key)
    }
}
impl New for ServiceMut<'_> {
    type Key = ItemKey;
    type This<'a> = ServiceMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        ServiceMut::new(sol, key)
    }
}
impl New for SkillMut<'_> {
    type Key = ItemKey;
    type This<'a> = SkillMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        SkillMut::new(sol, key)
    }
}
impl New for SubsystemMut<'_> {
    type Key = ItemKey;
    type This<'a> = SubsystemMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        SubsystemMut::new(sol, key)
    }
}
impl New for SwEffectMut<'_> {
    type Key = ItemKey;
    type This<'a> = SwEffectMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::This<'_> {
        SwEffectMut::new(sol, key)
    }
}
