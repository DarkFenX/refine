use crate::{api::FighterMut, misc::RearmMinion};

impl<'s> FighterMut<'s> {
    /// Force the fighter squad's rearm behavior.
    ///
    /// Solar system's default is used when override is not set.
    pub fn set_rearm_minion_override(&mut self, rearm_minion_override: Option<RearmMinion>) {
        let u_fighter = self.sol.u_data.items.get_mut(self.uid).dc_fighter_mut().unwrap();
        u_fighter.set_rearm_minion_override(rearm_minion_override);
    }
}
