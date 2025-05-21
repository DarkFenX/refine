use crate::sol::{ItemKey, SolarSystem, api::RigMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_rig(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_rig = uad_item.get_rig().unwrap();
        SolarSystem::util_remove_item(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_rig.get_fit_key());
        uad_fit.rigs.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> RigMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_rig(self.key);
    }
}
