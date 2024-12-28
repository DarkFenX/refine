use crate::{
    defs::{EEffectId, SolItemId},
    sol::svc::SolSvcs,
};

impl SolSvcs {
    pub(in crate::sol) fn is_effect_running(&self, item_id: &SolItemId, effect_id: &EEffectId) -> bool {
        self.running_effects.is_running(item_id, effect_id)
    }
}
