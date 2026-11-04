use crate::{
    Subsystem,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Subsystem<'s> {
    pub fn remove(self) {
        self.core.internal_remove_subsystem(self.uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_remove_subsystem(&mut self, subsystem_uid: UItemId) {
        CoreInner::util_remove_subsystem(&mut self.u_data, &mut self.svc, subsystem_uid, &mut self.reuse.eupdates);
        let u_subsystem = self.u_data.items.get(subsystem_uid).dc_subsystem().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_subsystem.get_fit_uid());
        u_fit.subsystems.remove(&subsystem_uid);
        self.internal_discard_item(subsystem_uid, UndoStep::SubsystemRemoved);
    }
}
