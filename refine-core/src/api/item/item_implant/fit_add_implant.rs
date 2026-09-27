use crate::{
    ad::AItemId,
    api::{FitMut, ImplantMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UFitId, UImplant, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_implant(&mut self, fit_uid: UFitId, type_aid: AItemId) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_implant = UImplant::new(item_id, type_aid, fit_uid, true, &self.u_data.r_data);
        let u_item = UItem::Implant(u_implant);
        let implant_uid = self.u_data.items.add(u_item);
        u_fit.implants.insert(implant_uid);
        SolarSystem::util_add_implant(&mut self.u_data, &mut self.svc, implant_uid, &mut self.cache.eupdates);
        implant_uid
    }
}

impl<'s> FitMut<'s> {
    pub fn add_implant(&mut self, type_id: ItemTypeId) -> ImplantMut<'_> {
        let implant_uid = self.sol.internal_add_implant(self.uid, type_id.into_aid());
        ImplantMut::new(self.sol, implant_uid)
    }
}
