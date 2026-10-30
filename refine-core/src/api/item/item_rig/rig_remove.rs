use crate::{
    api::RigMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_rig(&mut self, rig_uid: UItemId) {
        SolarSystem::util_remove_rig(&mut self.u_data, &mut self.svc, rig_uid, &mut self.cache.eupdates);
        let u_rig = self.u_data.items.get(rig_uid).dc_rig().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_rig.get_fit_uid());
        u_fit.rigs.remove(&rig_uid);
        self.internal_discard_item(rig_uid, UndoStep::RigRemoved);
    }
}

impl<'s> RigMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_rig(self.uid)
    }
}
