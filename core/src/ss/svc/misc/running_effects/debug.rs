use crate::{
    ss::{svc::debug, SsView},
    util::DebugResult,
};

use super::RunningEffects;

impl RunningEffects {
    pub(in crate::ss::svc) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for (item_id, effect_ids) in self.data.iter() {
            debug::check_item(ss_view, item_id)?;
            for effect_id in effect_ids.iter() {
                debug::check_effect(ss_view, effect_id)?;
            }
        }
        Ok(())
    }
}
