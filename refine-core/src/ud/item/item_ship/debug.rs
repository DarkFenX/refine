use crate::{
    dbg::DebugResult,
    ud::{UData, UShip},
};

impl UShip {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            base,
            fit_uid,
            // Not checked
            kind: _,
            physics: _,
        } = self;
        base.consistency_check(u_data)?;
        fit_uid.consistency_check(u_data)?;
        Ok(())
    }
}
