use crate::{
    dbg::{DebugResult, check_a_effect_id, check_fit_key},
    ud::{UData, UImplant},
};

impl UImplant {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for a_effect_id in reffs.iter() {
                check_a_effect_id(u_data, a_effect_id)?;
            }
        }
        check_fit_key(u_data, self.get_fit_key())?;
        Ok(())
    }
}
