use crate::{
    dbg::{DebugResult, check_effect_key},
    ud::{UData, USwEffect},
};

impl USwEffect {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for &effect_key in reffs.iter() {
                check_effect_key(u_data, effect_key)?;
            }
        }
        Ok(())
    }
}
