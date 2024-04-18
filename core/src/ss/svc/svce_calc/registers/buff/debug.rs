use crate::{
    ss::{
        svc::{
            debug::{check_attr, check_effect, check_item},
            svce_calc::debug::check_modifier,
        },
        SsView,
    },
    util::DebugResult,
};

use super::BuffRegister;

impl BuffRegister {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for (item_id, effect_ids) in self.effects.iter() {
            check_item(ss_view, item_id)?;
            for effect_id in effect_ids {
                check_effect(ss_view, effect_id)?;
            }
        }
        for ((item_id, attr_id), ss_mods) in self.modifiers.iter() {
            check_item(ss_view, item_id)?;
            check_attr(ss_view, attr_id)?;
            for ss_mod in ss_mods {
                check_modifier(ss_view, ss_mod)?;
            }
        }
        Ok(())
    }
}
