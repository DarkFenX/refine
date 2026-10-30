use crate::{
    ad::AItemId,
    api::{FitMut, ImplantMut, ItemTypeId},
    sol::{SolarSystem, UndoStep},
    ud::{UFitId, UImplant, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_implant(&mut self, fit_uid: UFitId, type_aid: AItemId) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_implant = UImplant::new(item_id, type_aid, fit_uid, true, &self.u_data.r_data);
        let u_item = UItem::Implant(u_implant);
        let implant_uid = self.u_data.items.add(u_item);
        self.attach_implant(fit_uid, implant_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::ImplantAdded(implant_uid));
        }
        implant_uid
    }
    pub(in crate::api) fn internal_reattach_implant(&mut self, implant_uid: UItemId) {
        self.u_data.items.reattach(implant_uid);
        let u_implant = self.u_data.items.get(implant_uid).dc_implant().unwrap();
        let fit_uid = u_implant.get_fit_uid();
        self.attach_implant(fit_uid, implant_uid);
    }
    fn attach_implant(&mut self, fit_uid: UFitId, implant_uid: UItemId) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.implants.insert(implant_uid);
        SolarSystem::util_add_implant(&mut self.u_data, &mut self.svc, implant_uid, &mut self.cache.eupdates);
    }
}

impl<'s> FitMut<'s> {
    pub fn add_implant(&mut self, type_id: ItemTypeId) -> ImplantMut<'_> {
        let implant_uid = self.sol.internal_add_implant(self.uid, type_id.into_aid());
        ImplantMut::new(self.sol, implant_uid)
    }
}
