use crate::sol::{ItemKey, SolarSystem, api::ServiceMut};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_service(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_service = uad_item.get_service().unwrap();
        self.svc.remove_item(&self.uad, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_service.get_fit_key());
        uad_fit.services.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ServiceMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_service(self.key);
    }
}
