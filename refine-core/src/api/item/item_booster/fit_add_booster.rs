use crate::{
    ad::AItemId,
    api::{BoosterMut, FitMut, ItemTypeId},
    sol::{SolarSystem, UndoStep},
    ud::{UBooster, UFitId, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_booster(&mut self, fit_uid: UFitId, type_aid: AItemId) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_booster = UBooster::new(item_id, type_aid, fit_uid, true, &self.u_data.r_data);
        let u_item = UItem::Booster(u_booster);
        let booster_uid = self.u_data.items.add(u_item);
        self.attach_booster(fit_uid, booster_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::BoosterAdded(booster_uid));
        }
        booster_uid
    }
    pub(in crate::api) fn internal_reattach_booster(&mut self, booster_uid: UItemId) {
        self.u_data.items.reattach(booster_uid);
        let u_booster = self.u_data.items.get(booster_uid).dc_booster().unwrap();
        let fit_uid = u_booster.get_fit_uid();
        self.attach_booster(fit_uid, booster_uid);
    }
    fn attach_booster(&mut self, fit_uid: UFitId, booster_uid: UItemId) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.boosters.insert(booster_uid);
        SolarSystem::util_add_booster(&mut self.u_data, &mut self.svc, booster_uid, &mut self.cache.eupdates);
    }
}

impl<'s> FitMut<'s> {
    pub fn add_booster(&mut self, type_id: ItemTypeId) -> BoosterMut<'_> {
        let booster_uid = self.sol.internal_add_booster(self.uid, type_id.into_aid());
        BoosterMut::new(self.sol, booster_uid)
    }
}
