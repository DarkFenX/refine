use crate::{
    dbg::DebugResult,
    ud::{UData, USubsystem},
};

impl USubsystem {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self { base, fit_uid } = self;
        base.consistency_check(u_data)?;
        fit_uid.consistency_check(u_data)?;
        Ok(())
    }
}
