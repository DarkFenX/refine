use crate::{EffectMode, SideEffect};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SideEffect<'s> {
    /// Set side effect state.
    ///
    /// Disabled side effects are not applied when parent item is in effect, while enabled do.
    pub fn set_state(&mut self, state: bool) {
        let effect_mode = match state {
            true => EffectMode::StateCompliance,
            false => EffectMode::FullCompliance,
        };
        self.sol
            .internal_set_effect_id_mode(self.item_uid, self.effect_aid, effect_mode);
    }
}
