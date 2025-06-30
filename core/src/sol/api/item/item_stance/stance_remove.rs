use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::StanceMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_stance(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_stance = uad_item.get_stance().unwrap();
        SolarSystem::util_remove_stance(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_stance.get_fit_key());
        uad_fit.stance = None;
        self.uad.items.remove(item_key);
    }
}

impl<'a> StanceMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_stance(self.key);
    }
}
