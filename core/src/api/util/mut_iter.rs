use lender::{Lender, Lending};

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
            uids: uids,
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
        let uid = *self.uids.get(self.index)?;
        self.index += 1;
        Some(T::new_new(self.sol, uid))
    }
}

pub trait RefFamily {
    type Ref<'a>;
}
pub(crate) trait New: RefFamily {
    type UId: Copy;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> <Self as RefFamily>::Ref<'_>;
}

// Implementations for non-item entities
impl RefFamily for FleetMut<'_> {
    type Ref<'a> = FleetMut<'a>;
}
impl New for FleetMut<'_> {
    type UId = UFleetId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        FleetMut::new(sol, uid)
    }
}
impl RefFamily for FitMut<'_> {
    type Ref<'a> = FitMut<'a>;
}
impl New for FitMut<'_> {
    type UId = UFitId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        FitMut::new(sol, uid)
    }
}
// Implementations for items
impl RefFamily for AutochargeMut<'_> {
    type Ref<'a> = AutochargeMut<'a>;
}
impl New for AutochargeMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        AutochargeMut::new(sol, uid)
    }
}
impl RefFamily for BoosterMut<'_> {
    type Ref<'a> = BoosterMut<'a>;
}
impl New for BoosterMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        BoosterMut::new(sol, uid)
    }
}
impl RefFamily for DroneMut<'_> {
    type Ref<'a> = DroneMut<'a>;
}
impl New for DroneMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        DroneMut::new(sol, uid)
    }
}
impl RefFamily for FighterMut<'_> {
    type Ref<'a> = FighterMut<'a>;
}
impl New for FighterMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        FighterMut::new(sol, uid)
    }
}
impl RefFamily for FwEffectMut<'_> {
    type Ref<'a> = FwEffectMut<'a>;
}
impl New for FwEffectMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        FwEffectMut::new(sol, uid)
    }
}
impl RefFamily for ImplantMut<'_> {
    type Ref<'a> = ImplantMut<'a>;
}
impl New for ImplantMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        ImplantMut::new(sol, uid)
    }
}
impl RefFamily for ProjEffectMut<'_> {
    type Ref<'a> = ProjEffectMut<'a>;
}
impl New for ProjEffectMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        ProjEffectMut::new(sol, uid)
    }
}
impl RefFamily for RigMut<'_> {
    type Ref<'a> = RigMut<'a>;
}
impl New for RigMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        RigMut::new(sol, uid)
    }
}
impl RefFamily for ServiceMut<'_> {
    type Ref<'a> = ServiceMut<'a>;
}
impl New for ServiceMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        ServiceMut::new(sol, uid)
    }
}
impl RefFamily for SkillMut<'_> {
    type Ref<'a> = SkillMut<'a>;
}
impl New for SkillMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        SkillMut::new(sol, uid)
    }
}
impl RefFamily for SubsystemMut<'_> {
    type Ref<'a> = SubsystemMut<'a>;
}
impl New for SubsystemMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        SubsystemMut::new(sol, uid)
    }
}
impl RefFamily for SwEffectMut<'_> {
    type Ref<'a> = SwEffectMut<'a>;
}
impl New for SwEffectMut<'_> {
    type UId = UItemId;
    fn new_new(sol: &mut SolarSystem, uid: Self::UId) -> Self::Ref<'_> {
        SwEffectMut::new(sol, uid)
    }
}
