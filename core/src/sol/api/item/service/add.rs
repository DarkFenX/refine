use crate::sol::{
    FitKey, ItemKey, ItemTypeId, SolarSystem,
    api::{FitMut, ServiceMut},
    uad::item::{ServiceState, UadItem, UadService},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_service(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: ServiceState,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_service = UadService::new(&self.uad.src, item_id, type_id, fit_key, state);
        let uad_item = UadItem::Service(uad_service);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.services.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_service(&mut self, type_id: ItemTypeId, state: ServiceState) -> ServiceMut {
        let item_key = self.sol.internal_add_service(self.key, type_id, state);
        ServiceMut::new(self.sol, item_key)
    }
}
