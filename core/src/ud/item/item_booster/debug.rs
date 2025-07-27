use crate::{
    dbg::{DebugResult, check_fit_key},
    ud::{UBooster, UData},
};

impl UBooster {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_key(u_data, self.get_fit_key())?;
        Ok(())
    }
}
