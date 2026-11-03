use crate::{RearmMinion, SolarSystem, sol::UndoStep};

impl SolarSystem {
    pub fn set_default_rearm_minions(&mut self, rearm_minions: RearmMinion) {
        let old_rearm_minions = std::mem::replace(&mut self.u_data.default_rearm_minions, rearm_minions);
        if old_rearm_minions != rearm_minions && self.undo.is_recording() {
            self.undo.record(UndoStep::DefaultRearmMinions(old_rearm_minions));
        }
    }
}
