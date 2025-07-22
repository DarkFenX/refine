use crate::{
    misc::FighterCountOverride,
    sol::{SolarSystem, api::FighterMut},
    uad::UadItemKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_count_override(
        &mut self,
        item_key: UadItemKey,
        count_override: Option<FighterCountOverride>,
    ) {
        // Update user data
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        let old_count = uad_fighter.get_count().map(|v| v.current);
        uad_fighter.set_count_override(count_override);
        let new_count = uad_fighter.get_count().map(|v| v.current);
        // Update services
        if old_count != new_count {
            let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
            self.svc.notify_fighter_count_changed(&self.uad, item_key, uad_fighter);
        }
    }
}

impl<'a> FighterMut<'a> {
    pub fn set_count_override(&mut self, count_override: Option<FighterCountOverride>) {
        self.sol.internal_set_fighter_count_override(self.key, count_override);
    }
}
