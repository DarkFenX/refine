use crate::{
    api::{Fit, FitMut, Item, ItemMut, MutIter},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'s> Fit<'s> {
    pub fn iter_items(&self) -> impl ExactSizeIterator<Item = Item<'_>> {
        iter_items(self.sol, self.uid)
    }
}

impl<'s> FitMut<'s> {
    pub fn iter_items(&self) -> impl ExactSizeIterator<Item = Item<'_>> {
        iter_items(self.sol, self.uid)
    }
    pub fn iter_items_mut(&mut self) -> MutIter<'_, ItemMut<'_>> {
        let item_uids = self.sol.u_data.get_fit_item_uids(self.uid);
        MutIter::new(self.sol, item_uids)
    }
}

fn iter_items(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Item<'_>> {
    let item_uids = sol.u_data.get_fit_item_uids(fit_uid);
    item_uids.into_iter().map(|item_uid| Item::new(sol, item_uid))
}
