use crate::{
    api::FighterMut,
    misc::{RearmMinions, StOption},
};

impl<'a> FighterMut<'a> {
    pub fn set_rearm_minions(&mut self, set_rearm_minions: StOption<RearmMinions>) {
        let u_fighter = self.sol.u_data.items.get_mut(self.uid).dc_fighter_mut().unwrap();
        u_fighter.set_rearm_minions(set_rearm_minions);
    }
}
