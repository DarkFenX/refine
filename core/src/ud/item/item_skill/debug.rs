use crate::{
    dbg::{DebugResult, check_effect_key, check_fit_key},
    ud::{UData, USkill},
};

impl USkill {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for &effect_key in reffs.iter() {
                check_effect_key(u_data, effect_key)?;
            }
        }
        check_fit_key(u_data, self.get_fit_key())?;
        Ok(())
    }
}
