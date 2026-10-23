use crate::{
    dbg::DebugResult,
    ud::{UData, USwEffect},
};

impl USwEffect {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self { base } = self;
        base.consistency_check(u_data)?;
        Ok(())
    }
}
