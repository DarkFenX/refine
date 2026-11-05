use crate::{
    Implant,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Implant<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_implant(self.uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_remove_implant(&mut self, implant_uid: UItemId) {
        SolCore::util_remove_implant(&mut self.u_data, &mut self.svc, implant_uid, &mut self.reuse.eupdates);
        let u_implant = self.u_data.items.get(implant_uid).dc_implant().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_implant.get_fit_uid());
        u_fit.implants.remove(&implant_uid);
        self.internal_discard_item(implant_uid, UndoStep::ImplantRemoved);
    }
}
