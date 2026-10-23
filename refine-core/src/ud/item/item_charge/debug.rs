use crate::{
    dbg::DebugResult,
    ud::{UCharge, UData},
};

impl UCharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            base,
            fit_uid,
            cont_item_uid,
            projs,
            // Not checked
            activated: _,
            force_disabled: _,
        } = self;
        base.consistency_check(u_data)?;
        fit_uid.consistency_check(u_data)?;
        cont_item_uid.consistency_check(u_data, false)?;
        projs.consistency_check(u_data)?;
        Ok(())
    }
}
