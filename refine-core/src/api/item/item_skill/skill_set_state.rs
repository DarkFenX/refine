use crate::{
    Skill,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Skill<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_skill_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_skill_state(&mut self, skill_uid: UItemId, state: bool) {
        self.internal_set_skill_state_inner::<false>(skill_uid, state);
    }
    fn internal_set_skill_state_inner<const SCREEN: bool>(&mut self, skill_uid: UItemId, state: bool) {
        let u_skill = self.u_data.items.get_mut(skill_uid).dc_skill_mut().unwrap();
        if SCREEN && state == u_skill.get_skill_state() {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::SkillState(skill_uid, u_skill.get_skill_state()));
        }
        let old_a_state = u_skill.get_state();
        u_skill.set_skill_state(state);
        let new_a_state = u_skill.get_state();
        u_skill.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolCore::util_switch_item_state(
            &mut self.u_data,
            &mut self.svc,
            skill_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
