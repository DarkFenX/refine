use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::ImplantMut},
    uad::UadEffectUpdates,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_implant(
        &mut self,
        item_key: ItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        let uad_implant = uad_item.get_implant().unwrap();
        SolarSystem::util_remove_implant(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_fit = self.uad.fits.get_mut(uad_implant.get_fit_key());
        uad_fit.implants.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> ImplantMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_implant(self.key, &mut reuse_eupdates);
    }
}
