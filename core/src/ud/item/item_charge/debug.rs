use crate::{
    dbg::{DebugResult, check_fit_id, check_item_id},
    ud::{UCharge, UData},
};

impl UCharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_id(u_data, self.get_fit_key())?;
        check_item_id(u_data, self.get_cont_item_key(), false)?;
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
