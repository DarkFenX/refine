use crate::{
    dbg::{DebugResult, check_effect_id},
    ud::{UData, item::misc::UEffectModes},
};

impl UEffectModes {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for &effect_key in self.by_key.keys() {
            check_effect_id(u_data, effect_key)?;
        }
        Ok(())
    }
}
