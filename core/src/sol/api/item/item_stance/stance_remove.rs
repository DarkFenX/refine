use crate::{
    sol::{SolarSystem, api::StanceMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_stance(
        &mut self,
        item_key: UadItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        let uad_stance = uad_item.get_stance().unwrap();
        SolarSystem::util_remove_stance(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_fit = self.uad.fits.get_mut(uad_stance.get_fit_key());
        uad_fit.stance = None;
        self.uad.items.remove(item_key);
    }
}

impl<'a> StanceMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_stance(self.key, &mut reuse_eupdates);
    }
}
