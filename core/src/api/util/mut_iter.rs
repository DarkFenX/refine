use lender::{Lender, Lending};

use crate::{
    api::{
        AutochargeMut, BoosterMut, DroneMut, FighterMut, FitMut, FleetMut, FwEffectMut, ImplantMut, ProjEffectMut,
        RigMut, ServiceMut, SkillMut, SubsystemMut, SwEffectMut,
    },
    sol::SolarSystem,
    ud::{UFitKey, UFleetKey, UItemKey},
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
    pub(in crate::api) fn new(sol: &'this mut SolarSystem, keys: Vec<T::Key>) -> Self {
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
    type Lend = <T as RefFamily>::Ref<'lend>;
}
impl<'iter, T> Lender for MutIter<'iter, T>
where
    T: New,
{
    fn next(&mut self) -> Option<<T as RefFamily>::Ref<'_>> {
        let key = *self.keys.get(self.index)?;
        self.index += 1;
        Some(T::new_new(self.sol, key))
    }
}

pub trait RefFamily {
    type Ref<'a>;
}
pub(crate) trait New: RefFamily {
    type Key: Copy;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> <Self as RefFamily>::Ref<'_>;
}

// Implementations for non-item entities
impl RefFamily for FleetMut<'_> {
    type Ref<'a> = FleetMut<'a>;
}
impl New for FleetMut<'_> {
    type Key = UFleetKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        FleetMut::new(sol, key)
    }
}
impl RefFamily for FitMut<'_> {
    type Ref<'a> = FitMut<'a>;
}
impl New for FitMut<'_> {
    type Key = UFitKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        FitMut::new(sol, key)
    }
}
// Implementations for items
impl RefFamily for AutochargeMut<'_> {
    type Ref<'a> = AutochargeMut<'a>;
}
impl New for AutochargeMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        AutochargeMut::new(sol, key)
    }
}
impl RefFamily for BoosterMut<'_> {
    type Ref<'a> = BoosterMut<'a>;
}
impl New for BoosterMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        BoosterMut::new(sol, key)
    }
}
impl RefFamily for DroneMut<'_> {
    type Ref<'a> = DroneMut<'a>;
}
impl New for DroneMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        DroneMut::new(sol, key)
    }
}
impl RefFamily for FighterMut<'_> {
    type Ref<'a> = FighterMut<'a>;
}
impl New for FighterMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        FighterMut::new(sol, key)
    }
}
impl RefFamily for FwEffectMut<'_> {
    type Ref<'a> = FwEffectMut<'a>;
}
impl New for FwEffectMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        FwEffectMut::new(sol, key)
    }
}
impl RefFamily for ImplantMut<'_> {
    type Ref<'a> = ImplantMut<'a>;
}
impl New for ImplantMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        ImplantMut::new(sol, key)
    }
}
impl RefFamily for ProjEffectMut<'_> {
    type Ref<'a> = ProjEffectMut<'a>;
}
impl New for ProjEffectMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        ProjEffectMut::new(sol, key)
    }
}
impl RefFamily for RigMut<'_> {
    type Ref<'a> = RigMut<'a>;
}
impl New for RigMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        RigMut::new(sol, key)
    }
}
impl RefFamily for ServiceMut<'_> {
    type Ref<'a> = ServiceMut<'a>;
}
impl New for ServiceMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        ServiceMut::new(sol, key)
    }
}
impl RefFamily for SkillMut<'_> {
    type Ref<'a> = SkillMut<'a>;
}
impl New for SkillMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        SkillMut::new(sol, key)
    }
}
impl RefFamily for SubsystemMut<'_> {
    type Ref<'a> = SubsystemMut<'a>;
}
impl New for SubsystemMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        SubsystemMut::new(sol, key)
    }
}
impl RefFamily for SwEffectMut<'_> {
    type Ref<'a> = SwEffectMut<'a>;
}
impl New for SwEffectMut<'_> {
    type Key = UItemKey;
    fn new_new(sol: &mut SolarSystem, key: Self::Key) -> Self::Ref<'_> {
        SwEffectMut::new(sol, key)
    }
}
