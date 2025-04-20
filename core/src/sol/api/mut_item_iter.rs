use crate::sol::{
    ItemKey, SolarSystem,
    api::{AutochargeMut, SkillMut},
};

use lender::{Lender, Lending};

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
pub enum SkillMutGenerator {}
impl New for SkillMutGenerator {
    type This<'a> = SkillMut<'a>;
    fn new_new(sol: &mut SolarSystem, key: ItemKey) -> Self::This<'_> {
        SkillMut::new(sol, key)
    }
}
