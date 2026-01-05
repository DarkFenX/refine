use crate::{
    dbg::{DebugResult, check_fit_uid},
    ud::{UData, UImplant},
};

impl UImplant {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_uid(u_data, self.get_fit_uid())?;
        Ok(())
    }
}
