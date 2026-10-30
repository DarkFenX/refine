use crate::{
    api::StanceMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_stance(&mut self, stance_uid: UItemId) {
        SolarSystem::util_remove_stance(&mut self.u_data, &mut self.svc, stance_uid, &mut self.cache.eupdates);
        let u_stance = self.u_data.items.get(stance_uid).dc_stance().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_stance.get_fit_uid());
        u_fit.stance = None;
        self.internal_discard_item(stance_uid, UndoStep::StanceRemoved);
    }
}

impl<'s> StanceMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_stance(self.uid);
    }
}
