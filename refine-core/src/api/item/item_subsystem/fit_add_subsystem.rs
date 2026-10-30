use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, SubsystemMut},
    sol::{SolarSystem, UndoStep},
    ud::{UFitId, UItem, UItemId, USubsystem},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_subsystem(&mut self, fit_uid: UFitId, type_aid: AItemId) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_subsystem = USubsystem::new(item_id, type_aid, fit_uid, true, &self.u_data.r_data);
        let u_item = UItem::Subsystem(u_subsystem);
        let subsystem_uid = self.u_data.items.add(u_item);
        self.attach_subsystem(fit_uid, subsystem_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::SubsystemAdded(subsystem_uid));
        }
        subsystem_uid
    }
    pub(in crate::api) fn internal_reattach_subsystem(&mut self, subsystem_uid: UItemId) {
        self.u_data.items.reattach(subsystem_uid);
        let u_subsystem = self.u_data.items.get(subsystem_uid).dc_subsystem().unwrap();
        let fit_uid = u_subsystem.get_fit_uid();
        self.attach_subsystem(fit_uid, subsystem_uid);
    }
    fn attach_subsystem(&mut self, fit_uid: UFitId, subsystem_uid: UItemId) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.subsystems.insert(subsystem_uid);
        SolarSystem::util_add_subsystem(&mut self.u_data, &mut self.svc, subsystem_uid, &mut self.cache.eupdates);
    }
}

impl<'s> FitMut<'s> {
    pub fn add_subsystem(&mut self, type_id: ItemTypeId) -> SubsystemMut<'_> {
        let subsystem_uid = self.sol.internal_add_subsystem(self.uid, type_id.into_aid());
        SubsystemMut::new(self.sol, subsystem_uid)
    }
}
