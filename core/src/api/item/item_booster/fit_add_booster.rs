use crate::{
    ad::AItemId,
    api::{BoosterMut, FitMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UBooster, UEffectUpdates, UFitId, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_booster(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_booster = UBooster::new(item_id, type_aid, fit_uid, true, &self.u_data.src);
        let u_item = UItem::Booster(u_booster);
        let booster_uid = self.u_data.items.add(u_item);
        u_fit.boosters.insert(booster_uid);
        SolarSystem::util_add_booster(&mut self.u_data, &mut self.svc, booster_uid, reuse_eupdates);
        booster_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn add_booster(&mut self, type_id: ItemTypeId) -> BoosterMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let booster_uid = self
            .sol
            .internal_add_booster(self.uid, type_id.into_aid(), &mut reuse_eupdates);
        BoosterMut::new(self.sol, booster_uid)
    }
}
