use crate::{
    dbg::DebugResult,
    ud::{UData, UStance},
};

impl UStance {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        self.get_fit_uid().consistency_check(u_data)?;
        Ok(())
    }
}
