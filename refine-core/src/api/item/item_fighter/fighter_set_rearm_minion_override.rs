use crate::{
    api::FighterMut,
    misc::RearmMinion,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fighter_rearm_minion_override(
        &mut self,
        fighter_uid: UItemId,
        rearm_minion_override: Option<RearmMinion>,
    ) {
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        if self.undo.is_recording() {
            let old_rearm_minion_override = u_fighter.get_rearm_minion_override();
            self.undo.record(UndoStep::FighterRearmMinionOverride(
                fighter_uid,
                old_rearm_minion_override,
            ));
        }
        u_fighter.set_rearm_minion_override(rearm_minion_override);
    }
}

impl<'s> FighterMut<'s> {
    /// Force the fighter squad's rearm behavior.
    ///
    /// Solar system's default is used when override is not set.
    pub fn set_rearm_minion_override(&mut self, rearm_minion_override: Option<RearmMinion>) {
        self.sol
            .internal_set_fighter_rearm_minion_override(self.uid, rearm_minion_override);
    }
}
