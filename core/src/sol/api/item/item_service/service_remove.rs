use crate::{
    sol::{SolarSystem, api::ServiceMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_service(
        &mut self,
        item_key: UadItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        let uad_service = uad_item.get_service().unwrap();
        SolarSystem::util_remove_service(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_fit = self.uad.fits.get_mut(uad_service.get_fit_key());
        uad_fit.services.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ServiceMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_service(self.key, &mut reuse_eupdates);
    }
}
