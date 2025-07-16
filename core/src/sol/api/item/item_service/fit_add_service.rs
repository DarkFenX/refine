use crate::{
    ad,
    def::{FitKey, ItemKey, ItemTypeId},
    misc::ServiceState,
    sol::{
        SolarSystem,
        api::{FitMut, ServiceMut},
    },
    uad::{UadEffectUpdates, UadItem, UadService},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_service(
        &mut self,
        fit_key: FitKey,
        a_item_id: ad::AItemId,
        state: ServiceState,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_service = UadService::new(item_id, a_item_id, fit_key, state, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::Service(uad_service);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.services.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_service(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_service(&mut self, type_id: ItemTypeId, state: ServiceState) -> ServiceMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self
            .sol
            .internal_add_service(self.key, type_id, state, &mut reuse_eupdates);
        ServiceMut::new(self.sol, item_key)
    }
}
