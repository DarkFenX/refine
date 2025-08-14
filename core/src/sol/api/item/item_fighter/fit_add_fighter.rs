use crate::{
    ad::AItemId,
    def::ItemTypeId,
    misc::{Coordinates, MinionState},
    sol::{
        SolarSystem,
        api::{FighterMut, FitMut},
    },
    ud::{UEffectUpdates, UFighter, UFitKey, UItem, UItemKey, UPosition},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fighter(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        state: MinionState,
        position: UPosition,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_fighter = UFighter::new(item_id, type_id, fit_key, state, position, &self.u_data.src);
        let u_item = UItem::Fighter(u_fighter);
        let item_key = self.u_data.items.add(u_item);
        u_fit.fighters.insert(item_key);
        // Add fighter and autocharges to services
        SolarSystem::util_add_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            item_key,
            reuse_eupdates,
        );
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fighter(
        &mut self,
        type_id: ItemTypeId,
        state: MinionState,
        coordinates: Option<Coordinates>,
    ) -> FighterMut<'_> {
        let mut u_position = UPosition::default();
        if let Some(coordinates) = coordinates {
            u_position.coordinates = coordinates.into();
        }
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self
            .sol
            .internal_add_fighter(self.key, type_id, state, u_position, &mut reuse_eupdates);
        FighterMut::new(self.sol, item_key)
    }
}
