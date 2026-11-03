use crate::{Booster, SolarSystem, sol::UndoStep, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Booster<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_booster(self.uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_remove_booster(&mut self, booster_uid: UItemId) {
        SolarSystem::util_remove_booster(&mut self.u_data, &mut self.svc, booster_uid, &mut self.reuse.eupdates);
        let u_booster = self.u_data.items.get(booster_uid).dc_booster().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_booster.get_fit_uid());
        u_fit.boosters.remove(&booster_uid);
        self.internal_discard_item(booster_uid, UndoStep::BoosterRemoved);
    }
}
