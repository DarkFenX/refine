use crate::{
    ss::{svc::debug, SsView},
    util::DebugResult,
};

use super::BuffRegister;

impl BuffRegister {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for (item_id, effect_ids) in self.effects.iter() {
            debug::check_item(ss_view, item_id)?;
            for effect_id in effect_ids.iter() {
                debug::check_effect(ss_view, effect_id)?;
            }
        }
        for ((item_id, attr_id), ss_mods) in self.modifiers.iter() {
            debug::check_item(ss_view, item_id)?;
            debug::check_attr(ss_view, attr_id)?;
            for ss_mod in ss_mods.iter() {}
        }
        Ok(())
    }
}
