use crate::{
    ad::AItemId,
    api::{FitMut, ImplantMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UImplant, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_implant(
        &mut self,
        fit_uid: UFitId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_implant = UImplant::new(item_id, type_id, fit_uid, true, &self.u_data.src);
        let u_item = UItem::Implant(u_implant);
        let implant_uid = self.u_data.items.add(u_item);
        u_fit.implants.insert(implant_uid);
        SolarSystem::util_add_implant(&mut self.u_data, &mut self.svc, implant_uid, reuse_eupdates);
        implant_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn add_implant(&mut self, type_id: ItemTypeId) -> ImplantMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let implant_uid = self
            .sol
            .internal_add_implant(self.uid, type_id.into_aid(), &mut reuse_eupdates);
        ImplantMut::new(self.sol, implant_uid)
    }
}
