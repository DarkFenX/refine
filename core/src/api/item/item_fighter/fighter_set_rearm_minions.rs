use crate::api::FighterMut;

impl<'a> FighterMut<'a> {
    pub fn set_rearm_minions(&mut self, rearm_minions: Option<bool>) {
        let u_fighter = self.sol.u_data.items.get_mut(self.uid).dc_fighter_mut().unwrap();
        u_fighter.set_rearm_minions(rearm_minions);
    }
}
