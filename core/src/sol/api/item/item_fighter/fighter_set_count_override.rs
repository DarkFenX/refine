use crate::{
    misc::FighterCountOverride,
    sol::{SolarSystem, api::FighterMut},
    ud::UItemKey,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_count_override(
        &mut self,
        item_key: UItemKey,
        count_override: Option<FighterCountOverride>,
    ) {
        // Update user data
        let u_fighter = self.u_data.items.get_mut(item_key).get_fighter_mut().unwrap();
        let old_count = u_fighter.get_count().map(|v| v.current);
        u_fighter.set_count_override(count_override);
        let new_count = u_fighter.get_count().map(|v| v.current);
        // Update services
        if old_count != new_count {
            let u_fighter = self.u_data.items.get(item_key).get_fighter().unwrap();
            self.svc.notify_fighter_count_changed(&self.u_data, item_key, u_fighter);
        }
    }
}

impl<'a> FighterMut<'a> {
    pub fn set_count_override(&mut self, count_override: Option<FighterCountOverride>) {
        self.sol.internal_set_fighter_count_override(self.key, count_override);
    }
}
