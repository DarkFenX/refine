use crate::{EffectMode, SideEffect};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SideEffect<'s> {
    /// Get side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn get_state(&self) -> bool {
        let effect_id = &self.effect_aid;
        let u_booster = self.core.u_data.items.get(self.item_uid).dc_booster().unwrap();
        match u_booster.get_effect_mode_by_aid(effect_id) {
            EffectMode::FullCompliance => false,
            EffectMode::StateCompliance => true,
            EffectMode::ForceRun => true,
            EffectMode::ForceStop => false,
        }
    }
}
