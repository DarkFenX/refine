use crate::{
    ad,
    def::ItemTypeId,
    misc::MinionState,
    sol::{
        SolarSystem,
        api::{FighterMut, FitMut},
    },
    ud::{UEffectUpdates, UFighter, UFitKey, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fighter(
        &mut self,
        fit_key: UFitKey,
        a_item_id: ad::AItemId,
        state: MinionState,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_fighter = UFighter::new(item_id, a_item_id, fit_key, state, &self.u_data.src, reuse_eupdates);
        let u_item = UItem::Fighter(u_fighter);
        let item_key = self.u_data.items.add(u_item);
        u_fit.fighters.insert(item_key);
        // Add fighter and autocharges to services
        let u_item = self.u_data.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        // Process autocharges
        SolarSystem::add_fighter_autocharges(&mut self.u_data, &mut self.svc, &mut self.rprojs, item_key);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fighter(&mut self, type_id: ItemTypeId, state: MinionState) -> FighterMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self
            .sol
            .internal_add_fighter(self.key, type_id, state, &mut reuse_eupdates);
        FighterMut::new(self.sol, item_key)
    }
}
