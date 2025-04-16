use crate::sol::{ItemKey, SolarSystem, api::BoosterMut};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_booster(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_booster = uad_item.get_booster().unwrap();
        self.svc.remove_item(&self.uad, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_booster.get_fit_key());
        uad_fit.boosters.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> BoosterMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_booster(self.key);
    }
}
