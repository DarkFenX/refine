use crate::{
    dbg::DebugResult,
    ud::{UData, item::misc::UEffectModes},
};

impl UEffectModes {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for effect_rid in self.by_rid.keys() {
            effect_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
