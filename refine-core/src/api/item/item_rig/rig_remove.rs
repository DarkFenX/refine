use crate::{
    Rig,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Rig<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_rig(self.uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_remove_rig(&mut self, rig_uid: UItemId) {
        CoreInner::util_remove_rig(&mut self.u_data, &mut self.svc, rig_uid, &mut self.reuse.eupdates);
        let u_rig = self.u_data.items.get(rig_uid).dc_rig().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_rig.get_fit_uid());
        u_fit.rigs.remove(&rig_uid);
        self.internal_discard_item(rig_uid, UndoStep::RigRemoved);
    }
}
