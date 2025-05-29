use crate::sol::{ItemKey, SolarSystem, api::ImplantMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_implant(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_implant = uad_item.get_implant().unwrap();
        SolarSystem::util_remove_implant(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_implant.get_fit_key());
        uad_fit.implants.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ImplantMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_implant(self.key);
    }
}
