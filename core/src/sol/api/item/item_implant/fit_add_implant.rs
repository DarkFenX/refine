use crate::{
    ad,
    def::{FitKey, ItemKey, ItemTypeId},
    sol::{
        SolarSystem,
        api::{FitMut, ImplantMut},
    },
    uad::{UadEffectUpdates, UadImplant, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_implant(
        &mut self,
        fit_key: FitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_implant = UadImplant::new(item_id, a_item_id, fit_key, true, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::Implant(uad_implant);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.implants.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_implant(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_implant(&mut self, type_id: ItemTypeId) -> ImplantMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.sol.internal_add_implant(self.key, type_id, &mut reuse_eupdates);
        ImplantMut::new(self.sol, item_key)
    }
}
