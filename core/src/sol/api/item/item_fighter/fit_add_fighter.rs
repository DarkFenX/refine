use crate::{
    ad,
    def::{FitKey, ItemKey, ItemTypeId},
    misc::MinionState,
    sol::{
        SolarSystem,
        api::{FighterMut, FitMut},
    },
    uad::{UadEffectUpdates, UadFighter, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fighter(
        &mut self,
        fit_key: FitKey,
        a_item_id: ad::AItemId,
        state: MinionState,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_fighter = UadFighter::new(item_id, a_item_id, fit_key, state, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::Fighter(uad_fighter);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.fighters.insert(item_key);
        // Add fighter and autocharges to services
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        // Process autocharges
        SolarSystem::add_fighter_autocharges(&mut self.uad, &mut self.svc, &mut self.rprojs, item_key);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fighter(&mut self, type_id: ItemTypeId, state: MinionState) -> FighterMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self
            .sol
            .internal_add_fighter(self.key, type_id, state, &mut reuse_eupdates);
        FighterMut::new(self.sol, item_key)
    }
}
