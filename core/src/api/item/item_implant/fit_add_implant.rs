use crate::{
    ad::AItemId,
    api::{FitMut, ImplantMut},
    def::ItemTypeId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitKey, UImplant, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_implant(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_implant = UImplant::new(item_id, type_id, fit_key, true, &self.u_data.src);
        let u_item = UItem::Implant(u_implant);
        let implant_key = self.u_data.items.add(u_item);
        u_fit.implants.insert(implant_key);
        SolarSystem::util_add_implant(&mut self.u_data, &mut self.svc, implant_key, reuse_eupdates);
        implant_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_implant(&mut self, type_id: ItemTypeId) -> ImplantMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let implant_key = self.sol.internal_add_implant(self.key, type_id, &mut reuse_eupdates);
        ImplantMut::new(self.sol, implant_key)
    }
}
