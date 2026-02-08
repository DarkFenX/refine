use crate::api::FighterMut;

impl<'a> FighterMut<'a> {
    pub fn set_rearm_minion(&mut self, rearm_minion: Option<bool>) {
        let u_fighter = self.sol.u_data.items.get_mut(self.uid).dc_fighter_mut().unwrap();
        u_fighter.set_rearm_minion(rearm_minion);
    }
}
