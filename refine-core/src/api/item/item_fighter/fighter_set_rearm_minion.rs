use crate::{api::FighterMut, misc::RearmMinion};

impl<'s> FighterMut<'s> {
    pub fn set_rearm_minion(&mut self, rearm_minion: Option<RearmMinion>) {
        let u_fighter = self.sol.u_data.items.get_mut(self.uid).dc_fighter_mut().unwrap();
        u_fighter.set_rearm_minion(rearm_minion);
    }
}
