use crate::{
    ad,
    sol::{
        FitKey, ItemKey, ItemTypeId, MinionState, SolarSystem,
        api::{FighterMut, FitMut},
        uad::item::{UadFighter, UadItem},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fighter(
        &mut self,
        fit_key: FitKey,
        a_item_id: ad::AItemId,
        state: MinionState,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_fighter = UadFighter::new(&self.uad.src, item_id, a_item_id, fit_key, state);
        let uad_item = UadItem::Fighter(uad_fighter);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.fighters.insert(item_key);
        // Add fighter and autocharges to services
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Process autocharges
        SolarSystem::add_fighter_autocharges(
            &mut self.uad,
            &mut self.svc,
            &mut self.reffs,
            &mut self.rprojs,
            item_key,
        );
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fighter(&mut self, type_id: ItemTypeId, state: MinionState) -> FighterMut {
        let item_key = self.sol.internal_add_fighter(self.key, type_id, state);
        FighterMut::new(self.sol, item_key)
    }
}
