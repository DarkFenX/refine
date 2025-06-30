use crate::{
    dbg::{DebugResult, check_a_effect_id, check_item_key},
    sol::reffs::REffs,
    uad::Uad,
};

impl REffs {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (&item_key, effect_ids) in self.data.iter() {
            check_item_key(uad, item_key, true)?;
            for effect_id in effect_ids {
                check_a_effect_id(uad, effect_id)?;
            }
        }
        Ok(())
    }
}
