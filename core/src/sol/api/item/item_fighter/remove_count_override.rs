use crate::sol::{ItemKey, SolarSystem, api::FighterMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fighter_count_override(&mut self, item_key: ItemKey) {
        // Update user data
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        let old_count = uad_fighter.get_count().map(|v| v.current);
        uad_fighter.set_count_override(None);
        let new_count = uad_fighter.get_count().map(|v| v.current);
        // Update services
        if old_count != new_count {
            let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
            self.svc.fighter_count_changed(&self.uad, item_key, uad_fighter);
        }
    }
}

impl<'a> FighterMut<'a> {
    pub fn remove_count_override(&mut self) {
        self.sol.internal_remove_fighter_count_override(self.key)
    }
}
