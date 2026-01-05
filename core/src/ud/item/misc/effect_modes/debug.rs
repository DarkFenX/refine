use crate::{
    dbg::{DebugResult, check_effect_rid},
    ud::{UData, item::misc::UEffectModes},
};

impl UEffectModes {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for &effect_rid in self.by_rid.keys() {
            check_effect_rid(u_data, effect_rid)?;
        }
        Ok(())
    }
}
