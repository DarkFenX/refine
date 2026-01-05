use crate::{
    dbg::{DebugResult, check_fit_uid, check_item_uid},
    ud::{UCharge, UData},
};

impl UCharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_uid(u_data, self.get_fit_uid())?;
        check_item_uid(u_data, self.get_cont_item_uid(), false)?;
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
