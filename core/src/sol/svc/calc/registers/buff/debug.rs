use crate::sol::{
    debug::{DebugResult, check_a_attr_id, check_a_effect_id, check_item_key},
    svc::calc::debug::check_raw_modifier,
    uad::Uad,
};

use super::BuffRegister;

impl BuffRegister {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (&item_key, effect_ids) in self.a_effect_ids.iter() {
            check_item_key(uad, item_key, true)?;
            for a_effect_id in effect_ids {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        for ((item_key, a_attr_id), raw_modifiers) in self.modifiers.iter() {
            check_item_key(uad, *item_key, true)?;
            check_a_attr_id(uad, a_attr_id)?;
            for raw_modifier in raw_modifiers {
                check_raw_modifier(uad, raw_modifier)?;
            }
        }
        Ok(())
    }
}
