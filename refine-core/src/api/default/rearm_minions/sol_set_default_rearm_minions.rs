use crate::{
    RearmMinion, Sol,
    sol::{CoreInner, UndoStep},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn set_default_rearm_minions(&mut self, rearm_minions: RearmMinion) {
        self.inner_mut().internal_set_default_rearm_minions(rearm_minions);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_default_rearm_minions(&mut self, rearm_minions: RearmMinion) {
        let old_rearm_minions = std::mem::replace(&mut self.u_data.default_rearm_minions, rearm_minions);
        if old_rearm_minions != rearm_minions && self.undo.is_recording() {
            self.undo.record(UndoStep::DefaultRearmMinions(old_rearm_minions));
        }
    }
}
