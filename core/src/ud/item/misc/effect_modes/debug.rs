use crate::{
    dbg::{DebugResult, check_effect_key},
    ud::{UData, item::misc::EffectModes},
};

impl EffectModes {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for &effect_key in self.by_key.keys() {
            check_effect_key(u_data, effect_key)?;
        }
        Ok(())
    }
}
