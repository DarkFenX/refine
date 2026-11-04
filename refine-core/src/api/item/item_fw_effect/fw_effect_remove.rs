use crate::{
    FwEffect,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FwEffect<'s> {
    pub fn remove(self) {
        self.core.internal_remove_fw_effect(self.uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_remove_fw_effect(&mut self, fw_effect_uid: UItemId) {
        CoreInner::util_remove_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_uid, &mut self.reuse.eupdates);
        let u_fw_effect = self.u_data.items.get(fw_effect_uid).dc_fw_effect().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_fw_effect.get_fit_uid());
        u_fit.fw_effects.remove(&fw_effect_uid);
        self.internal_discard_item(fw_effect_uid, UndoStep::FwEffectRemoved);
    }
}
