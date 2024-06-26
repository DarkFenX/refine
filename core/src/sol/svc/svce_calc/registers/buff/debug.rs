use crate::{
    sol::{
        svc::{
            debug::{check_attr, check_effect, check_item},
            svce_calc::debug::check_raw_modifier,
        },
        SolView,
    },
    util::DebugResult,
};

use super::SolBuffRegister;

impl SolBuffRegister {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for (item_id, effect_ids) in self.effects.iter() {
            check_item(sol_view, item_id)?;
            for effect_id in effect_ids {
                check_effect(sol_view, effect_id)?;
            }
        }
        for ((item_id, attr_id), raw_modifiers) in self.modifiers.iter() {
            check_item(sol_view, item_id)?;
            check_attr(sol_view, attr_id)?;
            for raw_modifier in raw_modifiers {
                check_raw_modifier(sol_view, raw_modifier)?;
            }
        }
        Ok(())
    }
}
