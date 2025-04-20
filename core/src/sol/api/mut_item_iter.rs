use lender::{Lender, Lending};

use crate::sol::{
    ItemKey, SolarSystem,
    api::{
        AutochargeMut, BoosterMut, DroneMut, FighterMut, FwEffectMut, ImplantMut, RigMut, ServiceMut, SkillMut,
        SubsystemMut,
    },
};

pub struct ItemMutIter<'this, T>
where
    T: New,
{
    sol: &'this mut SolarSystem,
    item_keys: Vec<ItemKey>,
    index: usize,
    phantom: std::marker::PhantomData<T>,
}
impl<'this, T> ItemMutIter<'this, T>
where
    T: New,
{
    pub(in crate::sol::api) fn new(sol: &'this mut SolarSystem, item_keys: Vec<ItemKey>) -> Self {
        Self {
            sol,
            item_keys,
            index: 0,
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'iter, 'lend, T> Lending<'lend> for ItemMutIter<'iter, T>
where
    T: New,
{
    type Lend = <T as New>::This<'lend>;
}
impl<'iter, T> Lender for ItemMutIter<'iter, T>
where
    T: New,
{
    fn next(&mut self) -> Option<<T as New>::This<'_>> {
        let item_key = *self.item_keys.get(self.index)?;
        Some(T::new_new(self.sol, item_key))
    }
}

pub trait New {
    type This<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_>;
}
pub enum AutochargeMutGenerator {}
impl New for AutochargeMutGenerator {
    type This<'a> = AutochargeMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        AutochargeMut::new(sol, key)
    }
}
pub enum BoosterMutGenerator {}
impl New for BoosterMutGenerator {
    type This<'a> = BoosterMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        BoosterMut::new(sol, key)
    }
}
pub enum DroneMutGenerator {}
impl New for DroneMutGenerator {
    type This<'a> = DroneMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        DroneMut::new(sol, key)
    }
}
pub enum FighterMutGenerator {}
impl New for FighterMutGenerator {
    type This<'a> = FighterMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        FighterMut::new(sol, key)
    }
}
pub enum FwEffectMutGenerator {}
impl New for FwEffectMutGenerator {
    type This<'a> = FwEffectMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        FwEffectMut::new(sol, key)
    }
}
pub enum ImplantMutGenerator {}
impl New for ImplantMutGenerator {
    type This<'a> = ImplantMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        ImplantMut::new(sol, key)
    }
}
pub enum RigMutGenerator {}
impl New for RigMutGenerator {
    type This<'a> = RigMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        RigMut::new(sol, key)
    }
}
pub enum ServiceMutGenerator {}
impl New for ServiceMutGenerator {
    type This<'a> = ServiceMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        ServiceMut::new(sol, key)
    }
}
pub enum SkillMutGenerator {}
impl New for SkillMutGenerator {
    type This<'a> = SkillMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        SkillMut::new(sol, key)
    }
}
pub enum SubsystemMutGenerator {}
impl New for SubsystemMutGenerator {
    type This<'a> = SubsystemMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        SubsystemMut::new(sol, key)
    }
}
