use crate::{
    dbg::DebugResult,
    ud::{UData, item::base::UItemBaseMutable},
};

impl UItemBaseMutable {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        Ok(())
    }
}
