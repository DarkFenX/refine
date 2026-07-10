use crate::{
    dbg::DebugResult,
    ud::{UCharge, UData},
};

impl UCharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        self.get_fit_uid().consistency_check(u_data)?;
        self.get_cont_item_uid().consistency_check(u_data, false)?;
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
