use crate::sol::{ItemKey, SolarSystem, api::SubsystemMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_subsystem(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_subsystem = uad_item.get_subsystem().unwrap();
        self.svc.remove_item(&self.uad, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_subsystem.get_fit_key());
        uad_fit.subsystems.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> SubsystemMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_subsystem(self.key);
    }
}
